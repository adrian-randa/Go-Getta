use std::ops::DerefMut;

use diesel::{query_dsl::methods::{FilterDsl, FindDsl}, ExpressionMethods, QueryResult, RunQueryDsl};
use serde::Deserialize;

use crate::{db::DBConnection, error::{InternalServerError, InvalidKeyError, UserAlreadyExistsError, InvalidUsernameError}, models::{AccountKey, User}, schema::{account_keys::{self, key, used}, users}};


#[derive(Debug, Deserialize)]
pub struct AccountCreationCredentials {
    key: String,
    username: String,
    password: String,
}

pub async fn create_account(credentials: AccountCreationCredentials, connection: DBConnection) -> Result<impl warp::Reply, warp::Rejection> {
    // To avoid race conditions, we will lock the connection first and only release the lock at the very end
    let mut connection_lock = connection.lock().await;
    let connection_lock = connection_lock.deref_mut();

    if credentials.username.contains(['/', '\\']) {
        Err(InvalidUsernameError)?;
    }

    let creation_key: AccountKey = account_keys::table
        .find(credentials.key.clone())
        .first(connection_lock).map_err(|_| InvalidKeyError)?;

    if creation_key.is_used() {
        Err(InvalidKeyError)?;
    }

    let queried_user: Result<User, diesel::result::Error> = users::table.find(credentials.username.clone()).first(connection_lock);
    if queried_user.is_ok() {
        return Err(UserAlreadyExistsError.into())
    }

    let created_user = User::new(
        credentials.username.clone(),
        bcrypt::hash(credentials.password, 4).map_err(|_| warp::reject::custom(InternalServerError))?,
        credentials.username.clone(),
        "".into()
    );

    
    let _ = diesel::insert_into(users::table).values(created_user).execute(connection_lock);

    let _ = diesel::update(account_keys::table.filter(key.eq(credentials.key))).set(used.eq(true)).execute(connection_lock);

    Ok(warp::reply())
}