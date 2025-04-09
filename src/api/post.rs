use std::ops::DerefMut;

use crate::{db::DBConnection, error::{InsufficientPermissionsError, InternalServerError, InvalidSessionError, PostDoesNotExistError}, models::Post, schema::{posts, ratings::{self, post}}, validate_session_from_headers};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
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

    let new_post = Post::new(&user, post_data.body, post_data.appendage_id, None, parent_post.as_ref());
    let post_id = new_post.get_id();

    diesel::insert_into(posts::table)
        .values(new_post)
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

pub async fn delete_post(headers: warp::http::HeaderMap, connection: DBConnection, post_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let post_to_delete: Post = posts::table
        .find(post_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| PostDoesNotExistError)?;

    if user.get_username() != post_to_delete.get_creator() {
        Err(InsufficientPermissionsError)?;
    }

    let parent_post = post_to_delete.try_fetch_parent(connection.clone()).await;

    diesel::delete(&post_to_delete)
        .execute(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    if let Some(mut parent_post) = parent_post {
        parent_post.set_comments_amount_unchecked(parent_post.get_comments_amount() - 1);
        let _: Result<Post, _> = parent_post.save_changes(connection.lock().await.deref_mut());
    }

    tokio::spawn(post_deletion_cleanup(connection, post_to_delete));

    Ok(warp::reply())
}

async fn post_deletion_cleanup(connection: DBConnection, deleted_post: Post) {

    // Delete all ratings for the post that no loger
    let _ = diesel::delete(
        ratings::table.filter(post.eq(deleted_post.get_id()))
    ).execute(connection.lock().await.deref_mut());

}