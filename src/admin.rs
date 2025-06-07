use diesel::{sql_query, RunQueryDsl};
use serde::Deserialize;

use std::ops::DerefMut;

use super::DBConnection;


#[derive(Debug, Deserialize)]
pub struct RunSqlData {
    sql: String,
}

pub async fn run_sql(connection: DBConnection, data: RunSqlData) -> Result<impl warp::Reply, warp::Rejection> {

    sql_query(data.sql).execute(connection.lock().await.deref_mut()).map_err(|e| warp::reply::with_status(format!("Error: {:?}", e), warp::http::StatusCode::BAD_REQUEST));

    Ok(warp::reply::with_status("Executed!", warp::http::StatusCode::ACCEPTED))
}