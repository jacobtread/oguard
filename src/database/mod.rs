use log::{debug, error, info, warn};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database as SeaDatabase;
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

pub mod entities;
mod migration;

/// Testing seeding logic
#[cfg(test)]
mod seed;

// Re-exports of database types
pub use sea_orm::DatabaseConnection;
pub use sea_orm::DbErr;

/// Database error result type
pub type DbResult<T> = Result<T, DbErr>;

const DATABASE_PATH: &str = "data/app.db";
const DATABASE_PATH_URL: &str = "sqlite:data/app.db";

/// Connects to the database and applies the admin changes if
/// required, returning the database connection
pub async fn init() -> DatabaseConnection {
    debug!("Connected to database..");

    connect_database().await
}

/// Connects to the database
async fn connect_database() -> DatabaseConnection {
    let path = Path::new(&DATABASE_PATH);

    // Create path to database file if missing
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            create_dir_all(parent).expect("Unable to create parent directory for sqlite database");
        }
    }

    // Create the database if file is missing
    if !path.exists() {
        File::create(path).expect("Unable to create sqlite database file");
    }

    // Connect to database
    let connection = SeaDatabase::connect(DATABASE_PATH_URL)
        .await
        .expect("Unable to create database connection");

    // Run migrations
    if let Err(err) = Migrator::up(&connection, None).await {
        if let DbErr::Custom(custom_err) = err {
            if custom_err
                .contains("is missing, this migration has been applied but its file is missing")
            {
                // Forward migrations are not always a failure, so its just a warning
                warn!(
                    "It looks like your app.db has been used with a newer version \
                of oguard, you may encounter unexpected issues or bugs its \
                recommended that you backup your database before trying a new version: {}",
                    custom_err
                )
            }
        } else {
            // Other errors should be considered fatal
            panic!("Failed to run database migrations: {}", err);
        }
    }

    connection
}
