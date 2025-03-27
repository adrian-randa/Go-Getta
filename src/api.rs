use serde::Serialize;

use crate::{db::DBConnection, error::InvalidSessionError, models::User, validate_session_from_headers};

#[derive(Debug, Serialize)]
struct WhoAmIResponse {
    username: String,
    public_name: String,
    biography: String,
}

impl WhoAmIResponse {
    fn from_user(user: User) -> Self {
        Self { username: user.get_username(), public_name: user.get_public_name(), biography: user.get_biography() }
    }
}

pub async fn who_am_i(headers: warp::http::HeaderMap, connection: DBConnection) -> Result<impl warp::Reply, warp::Rejection> {
    let user = validate_session_from_headers(&headers, connection).await.ok_or(InvalidSessionError)?;

    Ok(warp::reply::json(&WhoAmIResponse::from_user(user)))
}