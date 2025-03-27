use std::{collections::HashMap, sync::Arc};

use warp::reject::{reject, Rejection};

use crate::{pages::PageStore, validate_session_from_headers, DBConnection};



pub async fn render(
    headers: warp::http::HeaderMap,
    params: HashMap<String, String>,
    page_store: Arc<PageStore>,
    connection: DBConnection
) -> Result<impl warp::Reply, warp::Rejection> {

    match validate_session_from_headers(&headers, connection.clone()).await {
        Some(_) => return Ok(Box::new(
            warp::reply::html(page_store.main_page.clone())
        )),
        None => return Ok(Box::new(
            warp::reply::html(page_store.login_page.clone())
        )),
    }
}