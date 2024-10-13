pub mod error;

use error::Error;
use hyper_util::rt::TokioIo;
use log::trace;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint, Server};
use v1beta1::device_plugin_server::{DevicePlugin, DevicePluginServer};
use v1beta1::registration_client::RegistrationClient;
use v1beta1::{DevicePluginOptions, RegisterRequest};

pub mod v1beta1 {
    pub const HEALTHY: &str = "Healthy";

    pub const UNHEALTHY: &str = "Unhealthy";

    pub const VERSION: &str = "v1beta1";

    pub const DEVICE_PLUGIN_PATH: &str = "/var/lib/kubelet/device-plugins";

    pub const KUBELET_SOCKET: &str = "/var/lib/kubelet/device-plugins/kubelet.sock";

    tonic::include_proto!("v1beta1");
}

// -----------------------------------------------------------------------------------------------

pub async fn kubelet_channel() -> Result<Channel, Error> {
    let channel = Endpoint::from_static("http://127.0.0.1")
        .connect_with_connector(tower::service_fn(|_| async {
            let stream = UnixStream::connect(v1beta1::KUBELET_SOCKET).await?;
            Ok::<_, std::io::Error>(TokioIo::new(stream))
        }))
        .await?;
    Ok(channel)
}

pub async fn plugin_channel(sock_name: &'static str) -> Result<Channel, Error> {
    let channel = Endpoint::from_static("http://127.0.0.1")
        .connect_with_connector(tower::service_fn(move |_| async move {
            let sock_path = Path::new(v1beta1::DEVICE_PLUGIN_PATH).join(sock_name);
            let stream = UnixStream::connect(&sock_path).await?;
            Ok::<_, std::io::Error>(TokioIo::new(stream))
        }))
        .await?;
    Ok(channel)
}

pub async fn serve(
    sock_name: &str,
    resource_name: &str,
    service: impl DevicePlugin,
) -> Result<(), Error> {
    let sock_path = Path::new(v1beta1::DEVICE_PLUGIN_PATH).join(sock_name);

    let listener = listen(&sock_path, service);
    tokio::pin!(listener);

    let register = register(sock_name, resource_name);
    tokio::pin!(register);

    let ctrlc = tokio::signal::ctrl_c();
    tokio::pin!(ctrlc);

    let mut result = Ok(());
    let mut complete_register = false;
    loop {
        tokio::select! {
            v = &mut listener => {
                if v.is_err() {
                    result = v;
                    break;
                }
            }
            v = &mut register, if !complete_register => {
                if v.is_err() {
                    result = v;
                    break;
                }

                complete_register = true;
            },
            _ = &mut ctrlc => {
                break;
            }
        }
    }

    fs::remove_file(&sock_path).map_err(Error::socket)?;

    result
}

async fn listen(sock_path: &Path, service: impl DevicePlugin) -> Result<(), Error> {
    let listener = UnixListener::bind(sock_path).map_err(Error::listen)?;
    let stream = UnixListenerStream::new(listener);

    trace!("{}", "Starting to listen");
    Server::builder()
        .add_service(DevicePluginServer::new(service))
        .serve_with_incoming(stream)
        .await?;
    Ok(())
}

async fn register(sock_name: &str, resource_name: &str) -> Result<(), Error> {
    tokio::time::sleep(Duration::from_millis(10)).await;

    let channel = kubelet_channel().await?;

    let request = tonic::Request::new(RegisterRequest {
        version: v1beta1::VERSION.to_string(),
        endpoint: sock_name.to_string(),
        resource_name: resource_name.to_string(),
        options: Some(DevicePluginOptions::default()),
    });

    trace!("{}", "Registration.register");
    RegistrationClient::new(channel).register(request).await?;
    Ok(())
}
