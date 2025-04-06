use diesel::{associations::HasTable, prelude::*};
use dotenvy::dotenv;
use tokio::select;
use warp::Filter;
use std::{collections::HashMap, env, fs, ops::DerefMut, sync::Arc};
use uuid::*;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use go_getta::{api::{file_upload::file_upload, post::create_post, public_space::public_space_query, rating::set_rating_state, user_data::get_user_data, who_am_i}, clean_database, create_account::create_account, db::{establish_connection, scan_for_keys, with_db_connection}, login::*, models::*, pages::{with_page_store, PageStore}, render::render, schema::sessions::{self, timestamp}};


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

#[tokio::main]
async fn main() {
    let connection = establish_connection();
    connection.lock().await.run_pending_migrations(MIGRATIONS).unwrap();

    scan_for_keys(connection.clone()).await;
    clean_database(connection.clone()).await;

    let page_store = PageStore::init();

    let public_route = warp::get()
        .and(warp::any())
        .and(warp::fs::dir("./public"));

    let main_route = warp::get()
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_page_store(page_store.clone()))
        .and(with_db_connection(connection.clone()))
        .and_then(render);

    let login_route = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db_connection(connection.clone()))
        .and_then(login);

    let create_account_route = warp::post()
        .and(warp::path("create_account"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db_connection(connection.clone()))
        .and_then(create_account);

    let who_am_i_route = warp::get()
        .and(warp::path("api"))
        .and(warp::path("who_am_i"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and_then(who_am_i);

    let create_post_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("create_post"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(create_post);

    let public_space_route = warp::get()
        .and(warp::path("api"))
        .and(warp::path("fetch_public_space"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(public_space_query);

    let file_upload_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("file_upload"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(file_upload);

    let get_user_data_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "get_user_data" / String))
        .and_then(get_user_data);

    let set_rating_state_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("set_rating_state"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(set_rating_state);

    let storage_route = warp::get()
        .and(warp::path("storage"))
        .and(warp::fs::dir(env::var("STORAGE_URL").unwrap()));

    
    let routes = public_route
        .or(main_route)
        .or(login_route)
        .or(create_account_route)
        .or(who_am_i_route)
        .or(create_post_route)
        .or(public_space_route)
        .or(file_upload_route)
        .or(get_user_data_route)
        .or(set_rating_state_route)
        .or(storage_route);

    select! {
        _ = warp::serve(routes).run(([0, 0, 0, 0], 7500)) => (),
        _ = tokio::signal::ctrl_c() => {},
    }
}

