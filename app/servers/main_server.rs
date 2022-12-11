use crate::services::*;
use anyhow::Result;
use framework::servers::*;

/// get_server generates a default server on port 3000 that
/// holds a simple static service for testing the backend.
///
/// # Errors
///
/// This function will return an error if there is an issue
/// adding a service to the server
///
pub fn get_server() -> Result<ServerType> {
    let mut new_server = TcpServer::new();

    new_server.configure("0.0.0.0", 3000);
    new_server.add_service("/", main_site::get_service())?;

    Ok(new_server.into())
}
