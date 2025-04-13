use std::ops::DerefMut;

use diesel::{query_dsl::methods::FindDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use warp::reject::{InvalidHeader, Reject};

use crate::{db::DBConnection, error::{InternalServerError, InvalidPasswordError, InvalidSessionError}, extract_cookie, models::{Session, User}, schema::{sessions, users}, validate_session_from_headers};

#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    username: String,
    password: String,
    expires: bool,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    session_id: String,
}

pub async fn login(login_credentials: LoginCredentials, connection: DBConnection) -> Result<impl warp::Reply, warp::Rejection> {
    let user: User = users::table
        .find(login_credentials.username)
        .first(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    if !user.verify_password(login_credentials.password).map_err(|_| InternalServerError)? {
        return Err(InvalidPasswordError.into())
    }

    let session = Session::open_for_user(user, login_credentials.expires);
    let session_id = session.get_id();

    let _ = diesel::insert_into(sessions::table)
        .values(session)
        .execute(connection.lock().await.deref_mut());

    Ok(warp::reply::with_header(warp::reply::json(
        &LoginResponse { session_id }
    ), "Access-Control-Allow-Origin", "*"))
}


pub async fn logout(headers: warp::http::HeaderMap, connection: DBConnection) -> Result<impl warp::Reply, warp::Rejection> {
    let cookie_jar = headers.get("cookie").ok_or(InvalidSessionError)?.to_str().map_err(|_| InvalidSessionError)?.to_string();
    let session_id = extract_cookie(cookie_jar, "session_id".into()).ok_or(InvalidSessionError)?;

    let _ = diesel::delete(sessions::table.find(session_id)).execute(connection.lock().await.deref_mut());

    Ok(warp::reply())
}