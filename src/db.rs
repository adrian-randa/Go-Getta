use std::{env, fs, ops::DerefMut, sync::Arc};

use diesel::{Connection, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;
use tokio::sync::Mutex;
use warp::Filter;

use crate::{models::AccountKey, schema::account_keys};

pub type DBConnection = Arc<Mutex<SqliteConnection>>;

pub fn establish_connection() -> DBConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Arc::new(Mutex::new(SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))))
}

pub fn with_db_connection(connection: DBConnection) -> impl Filter<Extract = (DBConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || connection.clone())
}

pub async fn scan_for_keys(connection: DBConnection) {
    dotenv().ok();

    let key_store_url = env::var("KEY_STORE_URL").unwrap();
    let dir = fs::read_dir(key_store_url).unwrap();

    let mut connection_lock = connection.lock().await;
    let connection_lock = connection_lock.deref_mut();

    for file in dir.filter_map(|f| f.ok()) {
        if file.file_name().to_str().unwrap().split(".").last() == Some("ggkey") {
            let key = fs::read_to_string(file.path()).unwrap();

            if key.len() > 32 {
                continue;
            }

            diesel::insert_into(account_keys::table).values(
                AccountKey::new(key)
            ).execute(connection_lock).unwrap();

            fs::remove_file(file.path()).unwrap();
        }
    }
    
}