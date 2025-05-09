use std::{collections::HashMap, ops::DerefMut};

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
use serde::Serialize;

use crate::{db::DBConnection, error::{InternalServerError, InvalidQueryError, InvalidSessionError}, models::{Post, Room, User}, schema::{memberships, posts, rooms, users}, validate_session_from_headers};

use super::{PostQueryResponse, RoomQueryResponse, UserQueryResponse};


#[derive(Debug, Serialize)]
enum SearchResultResponse {
    Users(Vec<UserQueryResponse>),
    Posts(Vec<PostQueryResponse>),
    Rooms(Vec<RoomQueryResponse>),
}

pub async fn fetch_search_posts(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;
    let username = user.get_username();

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;
    let search_term = query.get("query").ok_or(InvalidQueryError)?;

    let joined_rooms: Vec<String> = memberships::table
        .filter(memberships::user.eq(&username))
        .select(memberships::room)
        .load(connection.lock().await.deref_mut())
        .or::<()>(Ok(Vec::new())).unwrap();

    let posts: Vec<Post> = posts::table
        .filter(posts::room.is_null().or(posts::room.eq_any(joined_rooms)).or(posts::creator.eq(&username)))
        .filter(posts::body.like(format!("%{}%", search_term)))
        .offset(page * 20)
        .limit(20)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let mut response = Vec::with_capacity(posts.len());

    for post in posts {
        response.push(PostQueryResponse::from_post_for_user(post, &user, connection.clone()).await?);
    }

    Ok(warp::reply::json(&SearchResultResponse::Posts(response)))
}

pub async fn fetch_search_users(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let _user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;
    let search_term = query.get("query").ok_or(InvalidQueryError)?;

    let users: Vec<_> = users::table
        .filter(users::username.like(format!("%{}%", search_term)).or(users::public_name.like(format!("%{}%", search_term))))
        .offset(page * 20)
        .limit(20)
        .load_iter(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?
        .filter_map(|u: Result<User, _>| {
            let u = u.ok()?;

            Some(u.into())
        }).collect();

    Ok(warp::reply::json(&SearchResultResponse::Users(users)))
}

pub async fn fetch_search_rooms(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;
    let search_term = query.get("query").ok_or(InvalidQueryError)?;

    let rooms: Vec<Room> = rooms::table
        .filter(rooms::is_private.eq(false))
        .filter(rooms::name.like(format!("%{}%", search_term)))
        .offset(page * 20)
        .limit(20)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let mut response = Vec::with_capacity(rooms.len());

    for room in rooms {
        response.push(RoomQueryResponse::from_room_for_user(room, &user, connection.clone()).await);
    }

    Ok(warp::reply::json(&SearchResultResponse::Rooms(response)))
}