use std::{collections::HashMap, ops::DerefMut};

use diesel::{query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl}, BoolExpressionMethods, ExpressionMethods, RunQueryDsl, SelectableHelper};
use serde::{Serialize, Deserialize};
use warp::reject::InvalidQuery;

use crate::{db::DBConnection, error::{InternalServerError, InvalidQueryError, InvalidSessionError}, models::Post, schema::posts::{self, parent, room, timestamp}, validate_session_from_headers};



#[derive(Debug, Serialize)]
pub struct PublicSpaceQueryResponse {
    posts: Vec<Post>
}

pub async fn public_space_query(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    let _user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;

    let posts: Vec<Post> = diesel::QueryDsl::order_by(
        diesel::query_dsl::methods::OffsetDsl::offset(
            posts::table.select(Post::as_select())
                .filter(room.is_null().and(parent.is_null()))
                , 20 * page as i64),
            timestamp.desc()
        )
        .limit(20)
        .load(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    Ok(warp::reply::json(&posts))
}