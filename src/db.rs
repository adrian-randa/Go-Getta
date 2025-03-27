use std::{env, sync::Arc};

use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use tokio::sync::Mutex;
use warp::Filter;

pub type DBConnection = Arc<Mutex<SqliteConnection>>;

pub fn establish_connection() -> DBConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Arc::new(Mutex::new(SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))))
}

pub fn with_db_connection(connection: DBConnection) -> impl Filter<Extract = (DBConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || connection.clone())
}