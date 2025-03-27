use std::collections::HashMap;

use warp::reject::{reject, Rejection};

use crate::{validate_session_from_headers, DBConnection};



pub async fn render(
    headers: warp::http::HeaderMap,
    params: HashMap<String, String>,
    connection: DBConnection
) -> Result<impl warp::Reply, warp::Rejection> {

    match validate_session_from_headers(&headers, connection.clone()).await {
        Some(_) => todo!(),
        None => todo!(),
    }
    

    Ok(Box::new(warp::reply()))
}