use anyhow::{Context, Error, Result};

// Bring into scope the various components of the backend
mod servers;
mod services;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Start initializing the various components of the backend
    let mut servers = servers::init_services()
        .with_context(|| "Failed to start the back-end due to invalid configuration")?;

    // Actually execute the backend
    servers.run().await
}
