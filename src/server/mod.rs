use std::{io::Error, ops::Deref, cell::RefCell, sync::Arc};

// use async_graphql::{EmptySubscription, Schema};
use poem::{get, listener::TcpListener, Route, Server, web::{Data, headers::{ Cookie, HeaderMapExt }, Json}, http::{HeaderMap}, handler, EndpointExt, endpoint::StaticFilesEndpoint, Response};
// use async_graphql_poem::{ GraphQLRequest, GraphQLResponse };
use surreal_poem::{ SurrealConnection, SurrealSession, create_surreal_socket_endpoint };
// use self::api::{Query, Mut};
// use self::api_auth::{Token};

use poem_proxy::{proxy};
// use surrealdb::{Session, sql::{Array, Value}, Datastore};

mod api;
mod api_auth;
pub mod database;

fn get_available_port(starting_port: u16, ending_port: u16) -> Option<u16>
{
    (starting_port..ending_port).find(|port| {
        match std::net::TcpListener::bind(("127.0.0.1", *port))
        {
            Ok(_) => true,
            Err(_) => false,
        }
    })
}

// This function injects the user's token into graphql's context
// #[handler]
// async fn graphql_post_handler(
//     schema: Data<&Schema<Query, Mut, EmptySubscription>>,
//     req: GraphQLRequest,
//     headers: &HeaderMap,
// ) -> GraphQLResponse
// {
//     if let Some( cookies ) = headers.typed_get::<Cookie>()
//     {
//         if let Some( token ) = cookies.get( "token" )
//         {
//             // token        
//             schema.execute(
//                 req.0.data( 
//                     Token( token.to_owned() )
//                 )
//             ).await
//         }
//         else
//         {
//             schema.execute( req.0 ).await
//         }
//     }
//     else
//     {
//         schema.execute( req.0 ).await
//     }.into()
// }

// #[handler]
// pub async fn db_request( db: Data<&Arc<Datastore>>, ses: Data<&Session> ) -> Json<Value> {
//     // let ses = Session::for_db( "root", "Finances" );
//     // todo!();
//     let ast = "CREATE users:joeyme SET age=32, name='Joey', qualities=['dumb', 'young'];";
//     let res = db.execute( ast, &ses, None, false ).await;

//     let Ok( res ) = res else {
//         todo!();
//     };

//     let Ok( res ) = &res[ 0 ].result else {
//         todo!();
//     };

//     Json( res.clone() )
// }

pub async fn start_server( 
    db: SurrealConnection, 
    ses: SurrealSession 
    ) -> Result<(), std::io::Error>
{
    // let j = |query: &str| {
    //     db.execute( query, &ses, None, false )
    // };

    // let ast = "USE NS test DB test;";
    // let res = db.execute( ast, &ses, None, false ).await;
    // println!( "{:?}", res );

    // loop {
    //     let mut line = String::new();
    //     let Ok( query ) = std::io::stdin().read_line( &mut line ) else {
    //         return Err( std::io::Error::new(std::io::ErrorKind::Other, "Failed to read line!") )
    //     };

    //     let Ok( response ) = db.execute( &line, &ses, None, false ).await else {
    //         println!( "Failed to execute query!" );
    //         continue;
    //     };

    //     let Ok( j ) = &response[ 0 ].result else {
    //         println!( "Huh" );
    //         continue;
    //     };

    //     // let Ok( res.result() ) =  else {
    //     //     println!( "Failed to run query" );
    //     //     continue;
    //     // };additional CARRIAGE RETURN (\r/U+000D)).
    //     .data(server_key)
    //     .finish();

    let port = get_available_port(3000, 4000)
        .expect("Could not start server! No open ports found between 3000 and 4000!");

    println!("Starting budgeteer server on localhost:{}", port);

    let app = Route::new()
        // .at( "/graphql", get( api::graphql_playground ).post( graphql_post_handler ).data( schema ))
        // .nest( "/", StaticFilesEndpoint::new( "./src/frontend/build" ).index_file( "index.html" ) )
        // .nest( "/", ProxyEndpoint::new( "http://localhost:5173".to_owned() ) );
        .at( "/rpc", create_surreal_socket_endpoint( db, ses ) )
        .nest( "/site", proxy.data( "http://localhost:5173".to_owned() ) );

    Server::new(TcpListener::bind(("0.0.0.0", port)))
        .run(app)
        .await
}
