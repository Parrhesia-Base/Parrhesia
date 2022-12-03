use anyhow::{Error, Result};
use servers::Servers;

// Bring into scope the various components of the backend
mod servers;
mod services;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Start initializing the various components of the backend
    let mut servers = Servers::new();
    servers.run().await
}
