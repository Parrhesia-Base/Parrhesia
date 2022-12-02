use dirs::data_dir;
use std::{fs, path::PathBuf, sync::Arc};
use surreal_poem::{get_connection, initiate_auth_db, SurrealSession};

use parrhesia::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>
{
    // Get the database connection string
    let database_uri = {
        let mut path = PathBuf::new();

        if let Some(dir) = data_dir() {
            path.push(dir);
            path.push("Parrhesia");
        };

        // Place files in subdirectory
        path.push("db");

        // Create all necessary folders in path
        fs::create_dir_all(&path).unwrap();

        // Add the file name
        path.push("data");

        // Get database URI and create the connection object
        let uri = format!("file://{}", path.to_str().unwrap());
        println!("Database path: {}", uri);
        uri
    };
    
    let key_url = {
        let mut path = PathBuf::new();
        if let Some( dir ) = data_dir() {
            path.push(dir);
            path.push("Parrhesia");
        };
        
        path.push( "server_key" );
        
        path
    };

    let Ok( db ) = get_connection( &database_uri ).await else {
        panic!( "Cannot start backend service because the database connection to {} failed!", &database_uri);
    };
    let ses = SurrealSession::for_db("parrhesia", "parrhesia");
    initiate_auth_db(Arc::clone(&db), "parrhesia", "parrhesia").await;

    // Start the server and database connection
    server::start_server(db, ses, key_url).await
}
