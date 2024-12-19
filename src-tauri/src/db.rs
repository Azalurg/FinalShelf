use std::path::Path;
use std::{env, fs};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").unwrap_or("./finalshelf.sql".to_string());

    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn init() {
    // This function is designed for readability.
    // In the future, we might check if the database exists and perform some advanced operations during initialization.
    run_migrations();
}

fn run_migrations() {
    let mut connection = establish_connection();
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
