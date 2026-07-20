use std::env;

use diesel::sqlite::SqliteConnection;
use diesel::{connection::SimpleConnection, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").unwrap_or("finalshelf.sql".to_string());

    let mut connection =
        SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // Reduce transient lock failures when multiple short-lived connections are created.
    let _ = connection.batch_execute("PRAGMA busy_timeout = 5000;");

    connection
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
