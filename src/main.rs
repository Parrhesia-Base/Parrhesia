mod server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>
{
    server::start_server().await
}
