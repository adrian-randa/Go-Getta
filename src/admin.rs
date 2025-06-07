use diesel::{sql_query, RunQueryDsl};
use serde::Deserialize;
use warp::reject::Reject;

use std::ops::DerefMut;

use super::DBConnection;


#[derive(Debug, Deserialize)]
pub struct RunSqlData {
    sql: String,
}

#[derive(Debug)]
struct RunSqlError {
    message: String,
}

impl Reject for RunSqlError {}

pub async fn run_sql(connection: DBConnection, data: RunSqlData) -> Result<impl warp::Reply, warp::Rejection> {

    sql_query(data.sql).execute(connection.lock().await.deref_mut()).map_err(|e| RunSqlError{ message: e.to_string() })?;

    Ok(warp::reply::with_status("Executed!", warp::http::StatusCode::ACCEPTED))
}