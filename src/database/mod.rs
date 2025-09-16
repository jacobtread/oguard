//! # Database
//!
//! Database connection logic, entity models, database specific logic and
//! a pattern similar to the repository pattern for working with database
//! data

use log::{debug, warn};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database as SeaDatabase;
use std::{
    fs::{File, create_dir_all},
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

/// Linux release builds save the db file to /usr/local/share/oguard/app.db
#[cfg(all(target_os = "linux", not(debug_assertions)))]
const DATABASE_PATH: &str = "/usr/local/share/oguard/app.db";

/// Macos release builds save the db file to /Library/Application Support/oguard/app.db
#[cfg(all(target_os = "macos", not(debug_assertions)))]
const DATABASE_PATH: &str = "/Lirbary/Application Support/oguard/app.db";

/// Windows and debug builds the db file is saved to the working directory
#[cfg(any(windows, debug_assertions))]
const DATABASE_PATH: &str = "data/app.db";

/// Connects to the database and applies the admin changes if
/// required, returning the database connection
pub async fn init() -> DatabaseConnection {
    debug!("Connected to database..");

    let path = Path::new(&DATABASE_PATH);

    // Create path to database file if missing
    if let Some(parent) = path.parent()
        && !parent.exists() {
            create_dir_all(parent).expect("Unable to create parent directory for sqlite database");
        }

    // Create the database if file is missing
    if !path.exists() {
        File::create(path).expect("Unable to create sqlite database file");
    }

    let db_url = format!("sqlite:{}", DATABASE_PATH);

    connect_database(&db_url).await
}

/// Connects to the database
pub async fn connect_database(url: &str) -> DatabaseConnection {
    // Connect to database
    let connection = SeaDatabase::connect(url)
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
