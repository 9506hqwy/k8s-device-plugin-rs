use device_plugin::error::Error;
use sample_device::{DevicePluginService, RESOURCE_NAME, SOCK_NAME, discover};
use tokio::sync::watch;

#[allow(clippy::result_large_err)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let (_tx, rx) = watch::channel(discover());
    let plugin = DevicePluginService::new(rx);

    device_plugin::serve(SOCK_NAME, RESOURCE_NAME, plugin).await?;
    Ok(())
}
