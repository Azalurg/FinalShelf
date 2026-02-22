// Quick migration runner for Phase 2
// Runs pending migrations via diesel_migrations, then exits

use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").unwrap_or("./finalshelf.sql".to_string());
    let mut connection =
        SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection.run_pending_migrations(MIGRATIONS).unwrap();
    println!("✓ Migrations complete — schema.rs should now contain the 4 new columns");
}
