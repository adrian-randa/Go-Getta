use std::{collections::HashMap, ops::DerefMut};

use diesel::{result::Error::NotFound, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl, SelectableHelper};
use serde::{Serialize, Deserialize};
use urlencoding::decode_binary;

use crate::{db::DBConnection, error::{InternalServerError, InvalidBiographyError, InvalidPublicNameError, InvalidQueryError, InvalidSessionError, UserDoesNotExistError}, models::{Following, Post, Room, User}, schema::{follows, memberships, posts::{self, creator, timestamp}, rooms, users}, validate_session_from_headers};

use super::PostQueryResponse;

#[derive(Serialize)]
struct GetUserDataResponse {
    public_name: String,
    biography: String,
    is_followed: bool,
    followers: i32,
    followed: i32,
}


pub async fn get_user_data(headers: warp::http::HeaderMap, connection: DBConnection, username: String) -> Result<impl warp::Reply, warp::Rejection> {
    
    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let binary = decode_binary(&username.as_bytes());
    let username = String::from_utf8_lossy(&binary).into_owned();

    let queried_user: User = users::table
        .find(&username)
        .first(connection.lock().await.deref_mut()).map_err(|_| UserDoesNotExistError)?;

    let is_followed = match follows::table
        .find((user.borrow_username(), &username))
        .first::<Following>(connection.lock().await.deref_mut()) {
            Ok(_) => true,
            Err(NotFound) => false,
            Err(_) => {Err(InternalServerError)?}
        };

    Ok(warp::reply::json(&GetUserDataResponse {
        public_name: queried_user.get_public_name(),
        biography: queried_user.get_biography(),
        is_followed,
        followers: queried_user.get_follower_count(),
        followed: queried_user.get_followed_count(),
    }))
}

pub async fn users_posts_query(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>, username: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let binary = decode_binary(&username.as_bytes());
    let username = String::from_utf8_lossy(&binary).into_owned();

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;

    let joined_rooms: Vec<String> = memberships::table
        .filter(memberships::user.eq(user.borrow_username()))
        .inner_join(rooms::table)
        .select(memberships::room)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let posts: Vec<Post> = posts::table
        .filter(creator.eq(username))
        .filter(posts::room.is_null().or(posts::room.eq_any(joined_rooms)))
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

#[derive(Debug, Deserialize)]
pub struct UpdatePublicNameData {
    new_public_name: String,
}

pub async fn update_public_name(headers: warp::http::HeaderMap, connection: DBConnection, data: UpdatePublicNameData) -> Result<impl warp::Reply, warp::Rejection> {

    let mut user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let length = data.new_public_name.len();
    if length > 32 || length < 1 {
        Err(InvalidPublicNameError)?;
    }

    user.set_public_name_unchecked(data.new_public_name);
    let _: User = user.save_changes(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}

#[derive(Debug, Deserialize)]
pub struct UpdateBiographyData {
    new_biography: String,
}

pub async fn update_biography(headers: warp::http::HeaderMap, connection: DBConnection, data: UpdateBiographyData) -> Result<impl warp::Reply, warp::Rejection> {

    let mut user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    if data.new_biography.len() > 2048 {
        Err(InvalidBiographyError)?;
    }

    user.set_biography_unchecked(data.new_biography);

    let _: User = user.save_changes(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}