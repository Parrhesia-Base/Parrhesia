use crate::services;
use framework::servers::*;

pub fn get_server() -> ServerType {
    let mut new_server = TcpServer::new();

    new_server.configure("0.0.0.0", 3000);
    new_server.add_service("/", services::main_site::get_service());

    new_server.into()
}
