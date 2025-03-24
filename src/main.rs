use diesel::{associations::HasTable, prelude::*};
use dotenvy::dotenv;
use std::env;
use uuid::*;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use go_getta::models::*;


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

#[tokio::main]
async fn main() {
    use go_getta::schema::account_keys::dsl::*;

    let connection = &mut establish_connection();
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    


}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}