use std::ops::DerefMut;

use crate::{db::DBConnection, error::{InternalServerError, InvalidSessionError}, models::Post, schema::posts, validate_session_from_headers};

use diesel::{query_dsl::methods::FindDsl, RunQueryDsl};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostCreationData {
    body: String,
    appendage_id: Option<String>,
    room: Option<String>,
    parent: Option<String>,
}

pub async fn create_post(headers: warp::http::HeaderMap, connection: DBConnection, post_data: PostCreationData) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let post = Post::new(&user, post_data.body, post_data.appendage_id, None, None);

    diesel::insert_into(posts::table)
        .values(post)
        .execute(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}