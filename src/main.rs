use diesel::{associations::HasTable, prelude::*};
use dotenvy::dotenv;
use tokio::select;
use warp::Filter;
use std::{env, fs};
use uuid::*;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use go_getta::models::*;


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

#[tokio::main]
async fn main() {
    use go_getta::schema::account_keys::dsl::*;

    let connection = &mut establish_connection();
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let public_route = warp::get()
        .and(warp::any())
        .and(warp::fs::dir("./public"));

    let login_page = fs::read_to_string("./pages/login.html").unwrap();

    let main_route = warp::get()
        .and(warp::path::end())
        .map(move || {
            warp::reply::html(login_page.clone())
        });

    
    let routes = public_route
        .or(main_route);

    select! {
        _ = warp::serve(routes).run(([0, 0, 0, 0], 7500)) => (),
        _ = tokio::signal::ctrl_c() => {},
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}