use std::{collections::HashMap, ops::DerefMut};

use crate::{db::{with_db_connection, DBConnection}, error::{InternalServerError, InvalidQueryError, InvalidSessionError, UserDoesNotExistError, UserIsAlreadyFollowedError}, models::{Following, Notification, Post, User}, schema::{follows::{self, followed, follower}, memberships, posts::{self, creator, timestamp}, rooms, users}, validate_session_from_headers};

use diesel::{result::Error::NotFound, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl, SelectableHelper};
use serde::Serialize;

use super::{PostQueryResponse, UserQueryResponse};


pub async fn follow(headers: warp::http::HeaderMap, connection: DBConnection, username: String) -> Result<impl warp::Reply, warp::Rejection> {

    let mut user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let mut user_to_follow: User = users::table
        .find(username)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| UserDoesNotExistError)?;

    connection.lock().await.exclusive_transaction(|conn| {
        match follows::table
            .find((user.borrow_username(), user_to_follow.borrow_username()))
            .first::<Following>(conn) {
                Ok(_) => {Err(diesel::result::Error::NotFound)?},
                Err(NotFound) => {},
                Err(e) => {Err(e)?},
            }
        
        diesel::insert_into(follows::table)
            .values(Following::new(&user, &user_to_follow))
            .execute(conn)?;

        user_to_follow.set_follower_count_unchecked(user_to_follow.get_follower_count() + 1);
        user.set_followed_count_unchecked(user.get_followed_count() + 1);

        let _: User = user_to_follow.save_changes(conn)?;
        let _: User = user.save_changes(conn)?;

        tokio::spawn(follow_notification_rollout(user.clone(), user_to_follow.get_username(), connection.clone()));

        Ok(())
    }).map_err(|_: diesel::result::Error| InternalServerError)?;

        

    Ok(warp::reply())
}

async fn follow_notification_rollout(emitter: User, username: String, connection: DBConnection) {
    let _ = Notification::push_unchecked(
        "Follow".into(), 
        &emitter, 
        username, 
        format!("{} is now following you!", emitter.get_public_name()),
        format!("?view=profile&id={}", emitter.borrow_username()),
        connection
    ).await;
}

pub async fn unfollow(headers: warp::http::HeaderMap, connection: DBConnection, username: String) -> Result<impl warp::Reply, warp::Rejection> {

    let mut user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let mut user_to_unfollow: User = users::table
        .find(username)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| UserDoesNotExistError)?;

    connection.lock().await.exclusive_transaction(|conn| {
        let _ =  follows::table
            .find((user.borrow_username(), user_to_unfollow.borrow_username()))
            .first::<Following>(conn)?;
        
        diesel::delete(follows::table.find((user.borrow_username(), user_to_unfollow.borrow_username())))
            .execute(conn)?;

        user_to_unfollow.set_follower_count_unchecked(user_to_unfollow.get_follower_count() - 1);
        user.set_followed_count_unchecked(user.get_followed_count() - 1);

        let _: User = user_to_unfollow.save_changes(conn)?;
        let _: User = user.save_changes(conn)?;

        Ok(())
    }).map_err(|_: diesel::result::Error| InternalServerError)?;

    Ok(warp::reply())
}

#[derive(Debug, Serialize)]
pub struct IsFollowingResponse {
    is_following: bool,
}

pub async fn is_following(headers: warp::http::HeaderMap, connection: DBConnection, username: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let is_following = match follows::table
        .find((user.borrow_username(), username))
        .first::<Following>(connection.lock().await.deref_mut()) {
            Ok(_) => true,
            Err(NotFound) => false,
            Err(_) => {Err(InternalServerError)?}
        };

    Ok(warp::reply::json(&IsFollowingResponse { is_following }))
}


pub async fn fetch_followed_feed(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;

    let followed_users: Vec<String> = follows::table
        .filter(follower.eq(user.borrow_username()))
        .select(followed)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let available_rooms: Vec<String> = memberships::table
        .inner_join(rooms::table)
        .filter(memberships::user.eq(user.borrow_username()).or(rooms::is_private.eq(false)))
        .select(memberships::room)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let posts: Vec<Post> = posts::table
        .filter(creator.eq_any(followed_users))
        .filter(posts::room.is_null().or(posts::room.eq_any(available_rooms)))
        .order(timestamp.desc())
        .offset(page * 20)
        .limit(20)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let mut response = Vec::with_capacity(posts.len());

    for post in posts {
        response.push(PostQueryResponse::from_post_for_user(post, &user, connection.clone()).await?);
    }

    Ok(warp::reply::json(&response))
}


pub async fn fetch_followers(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;
    
    let followers_usernames: Vec<String> = follows::table
        .filter(followed.eq(user.borrow_username()))
        .offset(page * 20)
        .limit(20)
        .select(follower)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let mut response: Vec<UserQueryResponse> = Vec::with_capacity(followers_usernames.len());

    for f in followers_usernames {
        let follower_user: User = users::table
            .find(&f)
            .first(connection.lock().await.deref_mut())
            .map_err(|_| UserDoesNotExistError)?;

        response.push(follower_user.into())
    }

    Ok(warp::reply::json(&response))
}

pub async fn fetch_followed(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;
    
    let followed_usernames: Vec<String> = follows::table
        .filter(follower.eq(user.borrow_username()))
        .offset(page * 20)
        .limit(20)
        .select(followed)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let mut response: Vec<UserQueryResponse> = Vec::with_capacity(followed_usernames.len());

    for f in followed_usernames {
        let followed_user: User = users::table
            .find(&f)
            .first(connection.lock().await.deref_mut())
            .map_err(|_| UserDoesNotExistError)?;

        response.push(followed_user.into())
    }

    Ok(warp::reply::json(&response))
}