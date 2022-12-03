use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
mod tcp_server;
pub use tcp_server::*;

use crate::services::ServiceType;

#[async_trait]
#[enum_dispatch(ServerType)]
pub trait Server {
    fn add_service(&mut self, path: impl AsRef<str>, service: ServiceType);
    fn add_nested_service<E>(&mut self, path: impl AsRef<str>, service: E)
    where
        E: poem::IntoEndpoint,
        E::Endpoint: 'static;
    async fn serve(mut self) -> anyhow::Result<(), anyhow::Error>;
}

#[enum_dispatch]
pub enum ServerType {
    TcpServer,
}
