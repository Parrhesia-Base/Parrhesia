use std::{path::PathBuf, fs};
use dirs::data_dir;
use surreal_poem::{ get_connection, SurrealSession };

mod server;

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

    let Ok( db ) = get_connection( &database_uri, "parrhesia", "parrhesia" ).await else {
        todo!() // Add error handling for when database connection fails
    };
    let ses = SurrealSession::for_db( "root", "Finances" );

    // Start the server and database connection
    server::start_server( db, ses ).await
}
