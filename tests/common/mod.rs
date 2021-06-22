use std::net::SocketAddr;
use tokio;
use {{crate_name}}::{config, error::Result, startup};

pub async fn spawn_app() -> Result<SocketAddr> {
    std::env::set_var("RUN_ENV", "test"); //set the test environment so the right config is loaded
    let app_settings = config::Settings::new()?;
    let (addr, server) = startup::run(app_settings).await?;
    tokio::task::spawn(server);
    Ok(addr) // Return the bound address
}
