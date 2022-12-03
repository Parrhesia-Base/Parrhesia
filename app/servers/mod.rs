use std::pin::Pin;

use anyhow::{bail, Error, Result};
use framework::servers::{Server, ServerType};
use futures::future;

mod main_server;

pub struct Servers {
    servers: Vec<ServerType>,
}

impl Servers {
    pub fn new() -> Self {
        // Initialize the servers object
        let mut servers = Servers { servers: vec![] };

        // Add servers to the backend
        servers.servers.push(main_server::get_server());

        // Return the servers object
        servers
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let mut futures: Vec<
            Pin<Box<dyn futures::Future<Output = Result<(), anyhow::Error>> + std::marker::Send>>,
        > = vec![];

        for server in std::mem::take(&mut self.servers) {
            futures.push(server.serve());
        }

        while !futures.is_empty() {
            match future::select_all(futures).await {
                (Ok(()), _index, remaining) => futures = remaining,
                (Err(_e), _, _) => {
                    bail!("One of the servers failed! Shutting down the backend.")
                }
            }
        }

        Ok(())
    }
}
