use std::{ops::DerefMut, time};

use diesel::{prelude::*, sqlite};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{db::DBConnection, schema::posts};

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


#[derive(Debug, Queryable, Selectable, Insertable, Identifiable, AsChangeset)]
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

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_public_name(&self) -> String {
        self.public_name.clone()
    }

    pub fn get_biography(&self) -> String {
        self.biography.clone()
    }

    pub fn set_public_name_unchecked(&mut self, public_name: String) {
        self.public_name = public_name;
    }

    pub fn set_biography_unchecked(&mut self, biography: String) {
        self.biography = biography;
    }
}

#[derive(Debug, Queryable, Insertable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(User, foreign_key = username))]
#[diesel(primary_key(id))]
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

#[derive(Debug, Queryable, Insertable, Selectable, Associations, Identifiable, Serialize, Deserialize, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = creator))]
#[diesel(belongs_to(Room, foreign_key = room))]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    id: String,
    creator: String,
    body: String,
    timestamp: i64,
    rating: i32,
    appendage_id: Option<String>,
    room: Option<String>,
    parent: Option<String>,
    comments: i32,
    shares: i32,
    reposts: i32,
    bookmarks: i32,
    child: Option<String>,
}

impl Post {
    pub fn new(creator: &User, body: String, appendage_id: Option<String>, room: Option<&Room>, parent: Option<&Post>, child: Option<&Post>) -> Self {
        Post {
            id: Uuid::new_v4().to_string(), 
            creator: creator.get_username(), 
            body,
            timestamp: time::UNIX_EPOCH.elapsed().unwrap().as_secs().try_into().unwrap(),
            rating: 0,
            appendage_id,
            room: room.map(|r| r.get_id()),
            parent: parent.map(|p| p.get_id()),
            comments: 0, shares: 0, reposts: 0, bookmarks: 0,
            child: child.map(|p| p.get_id()),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_rating(&self) -> i32 {
        self.rating
    }

    pub fn set_rating_unchecked(&mut self, rating: i32) {
        self.rating = rating;
    }

    pub async fn try_get_parent(&self, connection: DBConnection) -> Option<Post> {
        let post: Post = posts::table
            .find(self.parent.clone()?)
            .first(connection.lock().await.deref_mut())
            .ok()?;

        Some(post)
    }

    pub async fn try_get_child(&self, connection: DBConnection) -> Option<Post> {
        let post: Post = posts::table
            .find(self.child.clone()?)
            .first(connection.lock().await.deref_mut())
            .ok()?;

        Some(post)
    }

    pub fn get_comments_amount(&self) -> i32 {
        self.comments
    }

    pub fn set_comments_amount_unchecked(&mut self, amount: i32) {
        self.comments = amount;
    }

    pub fn get_creator(&self) -> String {
        self.creator.clone()
    }

    pub fn get_child(&self) -> Option<String> {
        self.child.clone()
    }

    pub fn get_reposts_amount(&self) -> i32 {
        self.reposts
    }

    pub fn set_reposts_amount_unchecked(&mut self, reposts: i32) {
        self.reposts = reposts;
    }
}

#[derive(Debug, Queryable, Insertable, Selectable, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::rooms)]
pub struct Room {
    id: String,
    name: String,
    description: String,
    color: String,
    date_created: i64,
    is_private: bool,
}

impl Room {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Debug, Queryable, Identifiable, Insertable, Selectable, AsChangeset, Associations)]
#[diesel(belongs_to(User, foreign_key = user))]
#[diesel(belongs_to(Post, foreign_key = post))]
#[diesel(primary_key(user, post))]
#[diesel(table_name = crate::schema::ratings)]
pub struct Rating {
    user: String,
    post: String,
    is_upvote: bool,
}

impl Rating {
    pub fn is_upvote(&self) -> bool {
        self.is_upvote
    }

    pub fn new(user: &User, post: &Post, is_upvote: bool) -> Self {
        Self {
            user: user.get_username(),
            post: post.get_id(),
            is_upvote
        }
    }
}