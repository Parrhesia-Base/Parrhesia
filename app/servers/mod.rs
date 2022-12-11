use anyhow::{Context, Result};
use framework::servers::ServerList;

mod main_server;

/// init_services is where we officially execute the services that
/// make up our backend. Thing of it as registering the various components
/// we depend on.
pub fn init_services() -> Result<ServerList> {
    let mut servers = ServerList::new();

    servers.add(
        main_server::get_server()
            .with_context(|| "While adding main_server to the server list.")?,
    );

    Ok(servers)
}
