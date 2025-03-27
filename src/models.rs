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

    pub fn is_used(&self) -> bool {
        self.used
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}


#[derive(Debug, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(username))]
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

    pub fn verify_password(&self, password: String) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(password, &self.password)
    }
}

#[derive(Debug, Queryable, Insertable, Selectable, Associations, Identifiable)]
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

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_timestamp(&self) -> Option<i64> {
        self.timestamp
    }
}