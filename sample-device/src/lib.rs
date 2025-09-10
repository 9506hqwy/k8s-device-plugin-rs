use device_plugin::v1beta1::device_plugin_server::DevicePlugin;
use device_plugin::v1beta1::{
    self, AllocateRequest, AllocateResponse, ContainerAllocateResponse, Device,
    DevicePluginOptions, Empty, ListAndWatchResponse, PreStartContainerRequest,
    PreStartContainerResponse, PreferredAllocationRequest, PreferredAllocationResponse,
};
use log::trace;
use tokio::sync::{mpsc, watch};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub const RESOURCE_NAME: &str = "demo/sample-device";
pub const SOCK_NAME: &str = "sample-device.sock";

// -----------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct DevicePluginService {
    devices: watch::Receiver<Vec<Device>>,
}

#[tonic::async_trait]
impl DevicePlugin for DevicePluginService {
    async fn get_device_plugin_options(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<DevicePluginOptions>, Status> {
        trace!("{}", "DevicePlugin.get_device_plugin_options");
        Ok(Response::new(DevicePluginOptions::default()))
    }

    type ListAndWatchStream = ReceiverStream<Result<ListAndWatchResponse, Status>>;

    async fn list_and_watch(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<Self::ListAndWatchStream>, Status> {
        trace!("{}", "DevicePlugin.list_and_watch");
        let (tx, rx) = mpsc::channel(1);
        let mut devices = self.devices.clone();

        tokio::spawn(async move {
            trace!("{}", "DevicePlugin.list_and_watch.start");
            devices.mark_changed();
            while devices.changed().await.is_ok() {
                let response = ListAndWatchResponse {
                    devices: devices.borrow_and_update().clone(),
                };

                trace!("{}", "DevicePlugin.list_and_watch.send");
                if tx.send(Ok(response)).await.is_err() {
                    break;
                }
            }
            trace!("{}", "DevicePlugin.list_and_watch.end");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_preferred_allocation(
        &self,
        _: Request<PreferredAllocationRequest>,
    ) -> Result<Response<PreferredAllocationResponse>, Status> {
        trace!("{}", "DevicePlugin.get_preferred_allocation");
        unimplemented!()
    }

    async fn allocate(
        &self,
        request: Request<AllocateRequest>,
    ) -> Result<Response<AllocateResponse>, Status> {
        trace!("{}", "DevicePlugin.allocate");
        let mut responses = vec![];

        for req in request.get_ref().container_requests.as_slice() {
            let mut res = ContainerAllocateResponse::default();
            for id in req.devices_ids.as_slice() {
                res.envs
                    .insert(format!("SAMPLE_DEVICE{id}"), "1".to_string());
            }
            responses.push(res);
        }

        Ok(Response::new(AllocateResponse {
            container_responses: responses,
        }))
    }

    async fn pre_start_container(
        &self,
        _: Request<PreStartContainerRequest>,
    ) -> Result<Response<PreStartContainerResponse>, Status> {
        trace!("{}", "DevicePlugin.pre_start_container");
        unimplemented!()
    }
}

impl DevicePluginService {
    pub fn new(devices: watch::Receiver<Vec<Device>>) -> Self {
        DevicePluginService { devices }
    }
}

// -----------------------------------------------------------------------------------------------

pub fn discover() -> Vec<Device> {
    let mut devices = vec![];

    for id in 1..5 {
        devices.push(Device {
            health: v1beta1::HEALTHY.to_string(),
            id: id.to_string(),
            topology: None,
        });
    }

    devices
}
