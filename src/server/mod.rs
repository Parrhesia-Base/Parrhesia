use async_graphql::{EmptySubscription, Schema};
use poem::{get, listener::TcpListener, Route, Server, web::{Data, headers::{ Cookie, HeaderMapExt }}, http::{HeaderMap}, handler, EndpointExt, endpoint::StaticFilesEndpoint};
use async_graphql_poem::{ GraphQLRequest, GraphQLResponse };

use self::api::{Query, Mut};
use self::api_auth::{Token};

use poem_proxy::ProxyEndpoint;

mod api;
mod api_auth;
mod database;

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
#[handler]
async fn graphql_post_handler(
    schema: Data<&Schema<Query, Mut, EmptySubscription>>,
    req: GraphQLRequest,
    headers: &HeaderMap,
) -> GraphQLResponse
{
    if let Some( cookies ) = headers.typed_get::<Cookie>()
    {
        if let Some( token ) = cookies.get( "token" )
        {
            // token        
            schema.execute(
                req.0.data( 
                    Token( token.to_owned() )
                )
            ).await
        }
        else
        {
            schema.execute( req.0 ).await
        }
    }
    else
    {
        schema.execute( req.0 ).await
    }.into()
}

pub async fn start_server() -> Result<(), std::io::Error>
{
    // Generate secret key
    let server_key = api_auth::get_secret_key();

    // Connect to the database
    let db = database::start_connection().await;

    // Build API Schema
    let schema = 
        api::start_schema()
        .data(db)
        .data(server_key)
        .finish();

    let port = get_available_port(3000, 4000)
        .expect("Could not start server! No open ports found between 3000 and 4000!");

    println!("Starting budgeteer server on localhost:{}", port);

    let app = Route::new()
        .at( "/graphql", get( api::graphql_playground ).post( graphql_post_handler ).data( schema ))
        // .nest( "/", StaticFilesEndpoint::new( "./src/frontend/build" ).index_file( "index.html" ) )
        .nest( "/", ProxyEndpoint::new( "http://localhost:5173".to_owned() ) );

    Server::new(TcpListener::bind(("0.0.0.0", port)))
        .run(app)
        .await
}
