use std::{collections::HashMap, ops::DerefMut};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{db::DBConnection, error::{InternalServerError, InvalidQueryError, InvalidSessionError, PostDoesNotExistError}, models::Post, schema::posts::{self, parent, rating, timestamp}, validate_session_from_headers};

use super::PostQueryResponse;

const MAX_PARENT_POST_COUNT: usize = 10;


pub async fn get_thread(headers: warp::http::HeaderMap, connection: DBConnection, post_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await
        .ok_or(InvalidSessionError)?;

    let selected_post: Post = posts::table
        .find(post_id)
        .first(connection.lock().await.deref_mut()).map_err(|_| PostDoesNotExistError)?;

    let mut thread = Vec::with_capacity(MAX_PARENT_POST_COUNT + 1);
    thread.push(PostQueryResponse::from_post_for_user(selected_post, &user, connection.clone()).await);

    for i in 0..MAX_PARENT_POST_COUNT {
        if let Some(parent_post) = thread[i].get_post_ref().try_fetch_parent(connection.clone()).await {
            thread.push(PostQueryResponse::from_post_for_user(parent_post, &user, connection.clone()).await);
        } else {
            break;
        }
    }

    thread.reverse();


    Ok(warp::reply::json(&thread))
}

pub async fn comment_query(headers: warp::http::HeaderMap, connection: DBConnection, parent_post_id: String, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;

    let comments: Vec<Post> = posts::table
        .filter(parent.eq(parent_post_id))
        .order(rating.desc())
        .offset(20 * page)
        .limit(20)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let mut response = Vec::with_capacity(comments.len());

    for comment in comments {
        response.push(PostQueryResponse::from_post_for_user(comment, &user, connection.clone()).await);
    }

    Ok(warp::reply::json(&response))
}