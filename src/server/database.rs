use dirs::data_dir;
// use migration::{Migrator, MigratorTrait};
// use sea_orm::{Database, DatabaseConnection};
use std::{fs, path::PathBuf};
use surrealdb::{ Datastore, Error };

pub async fn start_connection() -> Result<Datastore, Error> {
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

    Ok( Datastore::new( &uri ).await? )

    // Create database connection
    // let connection = Database::connect(uri)
    //     .await
    //     .expect("Failed to open database!");

    // Run migrations on database
    // Migrator::down(&connection, None)
    //     .await
    //     .expect("Failed to reset database");
    // Migrator::up(&connection, None)
    //     .await
    //     .expect("Database migration failed!");

    // And return the connection!
    // connection

}

