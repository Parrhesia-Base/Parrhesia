use dirs::data_dir;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::{fs, path::PathBuf};

pub async fn start_connection() -> DatabaseConnection {
    let mut path = PathBuf::new();

    if let Some(dir) = data_dir() {
        path.push(dir);
        path.push("Budgeteer");
    };

    // Place files in subdirectory
    path.push("db");

    // Create all necessary folders in path
    fs::create_dir_all(&path).unwrap();

    // Add the file name
    path.push("data");

    // Get database URI and create the connection object
    let uri = format!("sqlite:{}?mode=rwc", path.to_str().unwrap());
    println!("Database path: {}", uri);

    // Create database connection
    let connection = Database::connect(uri)
        .await
        .expect("Failed to open database!");

    // Run migrations on database
    Migrator::down(&connection, None)
        .await
        .expect("Failed to reset database");
    Migrator::up(&connection, None)
        .await
        .expect("Database migration failed!");

    // And return the connection!
    connection
}
