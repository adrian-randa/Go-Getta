use std::{ops::DerefMut, time};

use db::DBConnection;
use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use models::{Session, User};
use schema::{sessions::{self, id, timestamp}, users};

pub mod models;
pub mod schema;

pub mod error;
pub mod render;
pub mod pages;
pub mod db;

pub mod login;
pub mod create_account;

pub mod api;


pub async fn validate_session_from_headers(headers: &warp::http::HeaderMap, connection: DBConnection) -> Option<User> {
    let cookie_jar = headers.get("cookie")?.to_str().ok()?.to_string();
    let session_id = extract_cookie(cookie_jar, "session_id".into())?;
    
    let now: i64 = time::UNIX_EPOCH.elapsed().ok()?.as_secs().try_into().ok()?;

    let mut connection_lock = connection.lock().await;
    let connection_lock = connection_lock.deref_mut();

    let session: Session = sessions::table
        .find(session_id)
        .first(connection_lock).ok()?;

    if session.get_timestamp().is_none_or(|t| now - t <= 600)  {
        // Renew session timeout
        let _ = diesel::update(FilterDsl::filter(sessions::table, id.eq(session.get_id()))).set(timestamp.eq(now));

        return Some(
            users::table
                .find(session.get_username())
                .first(connection_lock).ok()?
        )
    }

    None
}

pub async fn clean_database(connection: DBConnection) {
    diesel::delete(FilterDsl::filter(sessions::table, timestamp.is_not_null())).execute(connection.lock().await.deref_mut()).unwrap();
}

pub fn extract_cookie(cookie_jar: String, key: String) -> Option<String> {
    let lhs = cookie_jar.split_once(&format!("{}=", key))?.1.to_string();

    if let Some(parts) = lhs.split_once(';') {
        return Some(parts.0.to_string())
    }

    Some(lhs)
}