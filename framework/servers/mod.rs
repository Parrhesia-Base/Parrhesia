use std::pin::Pin;

use anyhow::{bail, Error, Result};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
mod tcp_server;
use futures::future;
pub use tcp_server::*;

use crate::services::ServiceType;

#[async_trait]
#[enum_dispatch(ServerType)]
pub trait Server {
    fn add_service(&mut self, path: impl AsRef<str>, service: ServiceType) -> Result<()>;
    // fn add_nested_service<E>(&mut self, path: impl AsRef<str>, service: E)
    // where
    //     E: poem::IntoEndpoint,
    //     E::Endpoint: 'static;
    async fn serve(mut self) -> anyhow::Result<(), anyhow::Error>;
}

#[enum_dispatch]
pub enum ServerType {
    TcpServer,
}

pub struct ServerList {
    servers: Vec<ServerType>,
}

impl ServerList {
    pub fn new() -> Self {
        ServerList { servers: vec![] }
    }

    pub fn add<'a>(&'a mut self, server: ServerType) -> &'a mut Self {
        self.servers.push(server);
        self
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
