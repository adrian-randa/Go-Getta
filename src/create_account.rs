use std::ops::DerefMut;

use diesel::{query_dsl::methods::{FilterDsl, FindDsl}, ExpressionMethods, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{db::DBConnection, error::{InternalServerError, InvalidKeyError, InvalidUsernameError, UserAlreadyExistsError}, login::{login, LoginCredentials}, models::{AccountKey, Session, User}, schema::{account_keys::{self, key, used}, sessions, users}};


#[derive(Debug, Deserialize)]
pub struct AccountCreationCredentials {
    key: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct CreateAccountResponse {
    session_id: String
}

pub async fn create_account(credentials: AccountCreationCredentials, connection: DBConnection) -> Result<impl warp::Reply, warp::Rejection> {

    if credentials.username.contains(['/', '\\', ' ', '@']) {
        Err(InvalidUsernameError)?;
    }

    let creation_key: AccountKey = match account_keys::table
        .find(credentials.key.clone())
        .first(connection.lock().await.deref_mut()) {
            Ok(k) => {k},
            Err(diesel::NotFound) => {Err(InvalidKeyError)?},
            Err(_) => {Err(InternalServerError)?}
        };

    if creation_key.is_used() {
        Err(InvalidKeyError)?;
    }

    let queried_user: Result<User, diesel::result::Error> = users::table.find(credentials.username.clone()).first(connection.lock().await.deref_mut());
    if queried_user.is_ok() {
        return Err(UserAlreadyExistsError.into())
    }

    let created_user = User::new(
        credentials.username.clone(),
        bcrypt::hash(credentials.password, 4).map_err(|_| warp::reject::custom(InternalServerError))?,
        credentials.username.clone(),
        "".into()
    );

    
    let _ = diesel::insert_into(users::table).values(&created_user).execute(connection.lock().await.deref_mut());

    let _ = diesel::update(account_keys::table.filter(key.eq(credentials.key))).set(used.eq(true)).execute(connection.lock().await.deref_mut());

    let session = Session::open_for_user(created_user, true);
    let session_id = session.get_id();

    let _ = diesel::insert_into(sessions::table)
        .values(session)
        .execute(connection.lock().await.deref_mut());

    Ok(warp::reply::json(&CreateAccountResponse{ session_id }))
}