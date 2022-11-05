use std::sync::Arc;

use surrealdb::{Datastore, Session};
use server::database;

mod server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>
{
    // Connect to the database
    let db: Datastore = match database::start_connection().await {
        Ok( db ) => db,
        Err( err ) => {
            todo!()
        }
    };
    
    // let p_db = ::new( db );
    let ses = Session::for_db( "root", "Finances" );

    // p_db
    let p_db = Arc::new( db );


    server::start_server( p_db, ses ).await
}
