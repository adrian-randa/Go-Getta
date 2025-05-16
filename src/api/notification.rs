use std::{collections::HashMap, ops::DerefMut};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::{db::DBConnection, error::{InternalServerError, InvalidQueryError, InvalidSessionError}, models::Notification, schema::notifications, validate_session_from_headers};

pub async fn fetch_notifications(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;

    let response: Vec<Notification> = notifications::table
        .filter(notifications::user.eq(user.borrow_username()))
        .order(notifications::timestamp.desc())
        .offset(page * 20)
        .limit(20)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    Ok(warp::reply::json(&response))
}

#[derive(Debug, Deserialize)]
pub struct DeleteNotificaitonsData {
    ids: Vec<String>,
}

pub async fn delete_notifications(headers: warp::http::HeaderMap, connection: DBConnection, data: DeleteNotificaitonsData) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    diesel::delete(
        notifications::table
            .filter(notifications::user.eq(user.borrow_username()))
            .filter(notifications::id.eq_any(data.ids))
    ).execute(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}