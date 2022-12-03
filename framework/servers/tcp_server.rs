// use crate::services::Service;

use crate::services::Service;

use super::ServiceType;

use std::net::SocketAddr;

use super::{async_trait, Server};
// use poem::{listener::TcpListener, Route, RouteMethod};
use axum::Router;

#[derive(Default)]
pub struct TcpServer {
    router: Router,
    addr: String,
    port: u16,
}

impl TcpServer {
    pub fn new() -> Self {
        TcpServer::default()
    }

    pub fn configure(&mut self, addr: impl Into<String>, port: u16) {
        self.addr = addr.into();
        self.port = port;
    }
}

#[async_trait]
impl Server for TcpServer {
    fn add_service(&mut self, path: impl AsRef<str>, service: ServiceType) {
        // let mut router = std::mem::take(&mut self.router).at(path, service.extract_endpoint());
        let router = std::mem::take(&mut self.router);
        let mut router = router.route(path.as_ref(), service.extract_service());
        std::mem::swap(&mut self.router, &mut router);
        // std::mem::swap(&mut self.router, &mut router);
    }

    // fn add_nested_service<E>(&mut self, path: impl AsRef<str>, service: E)
    // where
    //     E: poem::IntoEndpoint,
    //     E::Endpoint: 'static,
    // {
    //     let mut router = std::mem::take(&mut self.router);
    //     router = router.nest(path, service);
    //     std::mem::swap(&mut self.router, &mut router);
    // }

    async fn serve(mut self) -> anyhow::Result<(), anyhow::Error> {
        // poem::Server::new(TcpListener::bind((self.addr, self.port)))
        //     .run(self.router)
        //     .await?;
        let addr: SocketAddr = format!("{}:{}", self.addr, self.port).parse()?;
        axum::Server::bind(&addr)
            .serve(self.router.into_make_service())
            .await?;

        Ok(())
    }
}

// pub async fn dothis() -> Result<(), std::io::Error>
// {
//     // Get the database connection string
//     // let database_uri = {
//     //     let mut path = PathBuf::new();

//     //     if let Some(dir) = data_dir() {
//     //         path.push(dir);
//     //         path.push("Parrhesia");
//     //     };

//     //     // Place files in subdirectory
//     //     path.push("db");

//     //     // Create all necessary folders in path
//     //     fs::create_dir_all(&path).unwrap();

//     //     // Add the file name
//     //     path.push("data");

//     //     // Get database URI and create the connection object
//     //     let uri = format!("file://{}", path.to_str().unwrap());
//     //     println!("Database path: {}", uri);
//     //     uri
//     // };

//     // let key_url = {
//     //     let mut path = PathBuf::new();
//     //     if let Some( dir ) = data_dir() {
//     //         path.push(dir);
//     //         path.push("Parrhesia");
//     //     };

//     //     path.push( "server_key" );

//     //     path
//     // };

//     // let Ok( db ) = get_connection( &database_uri ).await else {
//     //     panic!( "Cannot start backend service because the database connection to {} failed!", &database_uri);
//     // };
//     // let ses = SurrealSession::for_db("parrhesia", "parrhesia");
//     // initiate_auth_db(Arc::clone(&db), "parrhesia", "parrhesia").await;

//     // // Start the server and database connection
//     // server::start_server(db, ses, key_url).await

//     // routing::home::hello
//     // let app = Route::new();//.at("/", get(index))
//     // .at( "/graphql", get( api::graphql_playground ).post( graphql_post_handler ).data( schema ))
//     // .nest( "/", StaticFilesEndpoint::new( "./src/frontend/build" ).index_file( "index.html" ) )
//     // .nest( "/", ProxyEndpoint::new( "http://localhost:5173".to_owned() ) );
//     // .at( "/rpc", surreal_socket( db, ses, key_path ) )
//     // .nest( "/", proxy.data( proxy_config ) );
//         // .at( "/:name", get( routing::home::hello ) );

//     // poem::Server::new(TcpListener::bind(("0.0.0.0", 3000)))
//     //     .run(app)
//     //     .await
// }
