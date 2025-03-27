use std::{fs, sync::Arc};

use warp::Filter;

pub struct PageStore {
    pub login_page: String,
    pub main_page: String,
}

impl PageStore {
    pub fn init() -> Arc<Self> {
        Arc::new(Self {
            login_page: fs::read_to_string("./pages/login.html").unwrap(),
            main_page: fs::read_to_string("./pages/gui.html").unwrap(),
        })
    }
}

pub fn with_page_store(page_store: Arc<PageStore>) -> impl Filter<Extract = (Arc<PageStore>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || page_store.clone())
}