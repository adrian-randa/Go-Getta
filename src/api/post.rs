use std::ops::DerefMut;

use crate::{db::DBConnection, error::{ContentTooLargeError, EmptyContentError, InsufficientPermissionsError, InternalServerError, InvalidSessionError, PostDoesNotExistError, RoomBoundaryViolationError, RoomDoesNotExistError}, models::{Membership, Post, Room}, schema::{memberships, posts, ratings::{self, post}, rooms}, validate_session_from_headers};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use serde::{Deserialize, Serialize};

use super::PostQueryResponse;

#[derive(Debug, Deserialize)]
pub struct PostCreationData {
    body: String,
    appendage_id: Option<String>,
    room: Option<String>,
    parent: Option<String>,
    child: Option<String>
}

#[derive(Debug, Serialize)]
struct PostCreationResponse {
    post_id: String,
}

pub async fn create_post(headers: warp::http::HeaderMap, connection: DBConnection, mut post_data: PostCreationData) -> Result<impl warp::Reply, warp::Rejection> {
    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    if post_data.body.split_ascii_whitespace().next().is_none() {
        Err(EmptyContentError)?;
    }
    if post_data.body.len() > 300 {
        Err(ContentTooLargeError)?;
    }

    let mut parent_post  = None;
    if let Some(parent_id) = post_data.parent {
        let p: Post = posts::table
            .find(parent_id)
            .first(connection.lock().await.deref_mut())
            .map_err(|_| PostDoesNotExistError)?;

        if let Some(r) = p.get_room() {
            let r: Room = rooms::table
                .find(r)
                .first(connection.lock().await.deref_mut())
                .map_err(|_| RoomDoesNotExistError)?;

            post_data.room = Some(r.get_id());
        }

        parent_post = Some(p);
    }

    let mut child_post = None;
    if let Some(child_id) = post_data.child {
        let p: Post = posts::table
            .find(child_id)
            .first(connection.lock().await.deref_mut())
            .map_err(|_| PostDoesNotExistError)?;

        if let Some(r) = p.get_room() {
            let r: Room = rooms::table
                .find(r)
                .first(connection.lock().await.deref_mut())
                .map_err(|_| RoomDoesNotExistError)?;

            if r.is_private() && post_data.room.as_ref().is_none_or(|room_id| room_id != &r.get_id()) {
                Err(RoomBoundaryViolationError)?;
            }
        }

        child_post = Some(p);
    }

    let mut contained_room = None;
    
    if let Some(id) =  post_data.room.as_ref() {
        contained_room = Some(rooms::table
            .find(id)
            .first::<Room>(connection.lock().await.deref_mut())
            .map_err(|_| InternalServerError)?);
    };

    let new_post = Post::new(&user, post_data.body, post_data.appendage_id, contained_room.as_ref(), parent_post.as_ref(), child_post.as_ref());
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

    if let Some(mut child_post) = child_post {
        child_post.set_reposts_amount_unchecked(
            child_post.get_reposts_amount() + 1
        );

        let _: Result<Post, _> = child_post.save_changes(connection.lock().await.deref_mut());
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

    let parent_post = post_to_delete.try_get_parent(connection.clone()).await;
    let child_post = post_to_delete.try_get_child(connection.clone()).await;

    diesel::delete(&post_to_delete)
        .execute(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    if let Some(mut parent_post) = parent_post {
        parent_post.set_comments_amount_unchecked(parent_post.get_comments_amount() - 1);
        let _: Result<Post, _> = parent_post.save_changes(connection.lock().await.deref_mut());
    }

    if let Some(mut child_post) = child_post {
        child_post.set_reposts_amount_unchecked(child_post.get_reposts_amount() - 1);
        let _: Result<Post, _> = child_post.save_changes(connection.lock().await.deref_mut());
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

pub async fn get_post(headers: warp::http::HeaderMap, connection: DBConnection, post_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;
    
    let (queried_post, contained_room): (Post, Room) = posts::table
        .find(post_id)
        .inner_join(rooms::table)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| PostDoesNotExistError)?;

    if contained_room.is_private() {
        if memberships::table
            .find((user.get_username(), contained_room.get_id()))
            .first::<Membership>(connection.lock().await.deref_mut())
            .is_err() {
                Err(RoomBoundaryViolationError)?;
            }
    }

    Ok(warp::reply::json(&PostQueryResponse::from_post_for_user(queried_post, &user, connection).await))
}