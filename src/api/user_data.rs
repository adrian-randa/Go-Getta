use std::ops::DerefMut;

use diesel::{query_dsl::methods::FindDsl, RunQueryDsl};
use serde::Serialize;

use crate::{db::DBConnection, error::{InvalidSessionError, UserDoesNotExistError}, models::User, schema::users, validate_session_from_headers};

#[derive(Serialize)]
struct GetUserDataResponse {
    public_name: String,
    biography: String
}


pub async fn get_user_data(headers: warp::http::HeaderMap, connection: DBConnection, username: String) -> Result<impl warp::Reply, warp::Rejection> {
    
    let _user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let queried_user: User = users::table
        .find(username)
        .first(connection.lock().await.deref_mut()).map_err(|_| UserDoesNotExistError)?;

    Ok(warp::reply::json(&GetUserDataResponse {
        public_name: queried_user.get_public_name(),
        biography: queried_user.get_biography()
    }))
}