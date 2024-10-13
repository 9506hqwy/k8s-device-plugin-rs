use device_plugin::error::Error;
use device_plugin::v1beta1::device_plugin_client::DevicePluginClient;
use device_plugin::v1beta1::Empty;
use log::trace;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let channel = device_plugin::plugin_channel(sample_device::SOCK_NAME).await?;

    trace!("{}", "DevicePlugin.list_and_watch");
    let mut response = DevicePluginClient::new(channel)
        .list_and_watch(Empty::default())
        .await?;

    while let Some(response) = response.get_mut().message().await? {
        for device in response.devices {
            println!("{:?}", device);
        }
    }

    Ok(())
}
