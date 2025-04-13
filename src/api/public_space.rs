use std::{collections::HashMap, ops::DerefMut};

use diesel::{query_dsl::methods::{FilterDsl, LimitDsl, OffsetDsl, OrderDsl, SelectDsl}, BoolExpressionMethods, ExpressionMethods, RunQueryDsl, SelectableHelper};
use serde::{Serialize, Deserialize};
use warp::reject::InvalidQuery;

use crate::{db::DBConnection, error::{InternalServerError, InvalidQueryError, InvalidSessionError}, models::Post, schema::posts::{self, parent, room, timestamp}, validate_session_from_headers};

use super::PostQueryResponse;

pub async fn public_space_query(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;

    let posts: Vec<Post> = posts::table
        .filter(room.is_null().and(parent.is_null()))
        .order(timestamp.desc())
        .offset(20 * page as i64)
        .limit(20)
        .load(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    let mut response = Vec::with_capacity(posts.len());

    for post in posts {
        response.push(PostQueryResponse::from_post_for_user(post, &user, connection.clone()).await);
    }

    Ok(warp::reply::json(&response))
}