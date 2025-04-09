use std::ops::DerefMut;

use crate::{db::DBConnection, error::{InternalServerError, InvalidSessionError, PostDoesNotExistError}, models::Post, schema::posts, validate_session_from_headers};

use diesel::{QueryDsl, RunQueryDsl, SaveChangesDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PostCreationData {
    body: String,
    appendage_id: Option<String>,
    room: Option<String>,
    parent: Option<String>,
}

#[derive(Debug, Serialize)]
struct PostCreationResponse {
    post_id: String,
}

pub async fn create_post(headers: warp::http::HeaderMap, connection: DBConnection, post_data: PostCreationData) -> Result<impl warp::Reply, warp::Rejection> {
    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let mut parent_post  = None;
    if let Some(parent_id) = post_data.parent {
        let p: Post = posts::table
            .find(parent_id)
            .first(connection.lock().await.deref_mut())
            .map_err(|_| PostDoesNotExistError)?;

        parent_post = Some(p);
    }

    let post = Post::new(&user, post_data.body, post_data.appendage_id, None, parent_post.as_ref());
    let post_id = post.get_id();

    diesel::insert_into(posts::table)
        .values(post)
        .execute(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    if let Some(mut parent_post) = parent_post {
        parent_post.set_comments_amount_unchecked(
            parent_post.get_comments_amount() + 1
        );

        let _: Result<Post, _> = parent_post.save_changes(connection.lock().await.deref_mut());
    }

    Ok(warp::reply::json(&PostCreationResponse {
        post_id
    }))
}