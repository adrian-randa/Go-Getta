use std::time;

use diesel::{prelude::*, sqlite};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::account_keys)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct AccountKey {
    key: String,
    used: bool,
}

impl AccountKey {
    pub fn new(key: String) -> AccountKey {
        Self { key, used: false }
    }
}


#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct User {
    username: String,
    password: String,
    public_name: String,
    biography: String,
}

impl User {
    pub fn new(username: String, password: String, public_name: String, biography: String) -> Self {
        Self { username, password, public_name, biography }
    }
}

#[derive(Debug, Queryable, Insertable, Selectable, Associations)]
#[diesel(belongs_to(User, foreign_key = username))]
#[diesel(table_name = crate::schema::sessions)]
pub struct Session {
    id: String,
    username: String,
    timestamp: Option<i64>
}

impl Session {

    pub fn open_for_user(user: User, expires: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username: user.username,
            timestamp: if expires {Some(time::UNIX_EPOCH.elapsed().unwrap().as_secs().try_into().unwrap())} else {None}
        }
    }
    
}