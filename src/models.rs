use std::{ops::DerefMut, time};

use diesel::{prelude::*, sqlite};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{api::room::RoomCreationData, db::DBConnection, error::{ContentTooLargeError, InternalServerError, CooldownActiveError}, schema::{notifications, posts, notification_timeouts}};

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


#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(username))]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct User {
    username: String,
    password: String,
    public_name: String,
    biography: String,
    followers: i32,
    followed: i32,
}

impl User {
    pub fn new(username: String, password: String, public_name: String, biography: String) -> Self {
        Self { username, password, public_name, biography, followers: 0, followed: 0 }
    }

    pub fn verify_password(&self, password: String) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(password, &self.password)
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn borrow_username(&self) -> &String {
        &self.username
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

    pub fn get_follower_count(&self) -> i32 {
        self.followers
    }

    pub fn set_follower_count_unchecked(&mut self, followers: i32) {
        self.followers = followers;
    }

    pub fn get_followed_count(&self) -> i32 {
        self.followed
    }

    pub fn set_followed_count_unchecked(&mut self, followed: i32) {
        self.followed = followed;
    }
}

#[derive(Debug, Queryable, Insertable, Selectable, Associations, Identifiable, AsChangeset)]
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

    pub fn renew(&mut self, timestamp: i64) {
        if let Some(t) = self.timestamp.as_mut() {
            *t = timestamp;
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable, Selectable, Associations, Identifiable, Serialize, Deserialize, AsChangeset)]
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

    pub fn get_room(&self) -> Option<String> {
        self.room.clone()
    }

    pub fn set_shares_unchecked(&mut self, shares: i32) {
        self.shares = shares;
    }

    pub fn get_shares(&self) -> i32 {
        self.shares
    }

    pub fn set_bookmarks_unchecked(&mut self, bookmarks: i32) {
        self.bookmarks = bookmarks;
    }

    pub fn get_bookmarks(&self) -> i32 {
        self.bookmarks
    }
}

#[derive(Debug, Queryable, Insertable, Selectable, Identifiable, Serialize, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User, foreign_key = owner))]
#[diesel(table_name = crate::schema::rooms)]
pub struct Room {
    id: String,
    name: String,
    description: String,
    color: String,
    date_created: i64,
    is_private: bool,
    owner: String
}

impl Room {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn try_open(data: RoomCreationData, user: &User) -> Result<Self, warp::Rejection> {
        if data.name.len() > 24 || data.description.len() > 150 || data.color.len() > 6 {
            Err(ContentTooLargeError)?;
        }

        Ok(Self {
            id: Uuid::new_v4().into(),
            name: data.name,
            description: data.description,
            color: data.color,
            date_created: time::UNIX_EPOCH.elapsed().unwrap().as_secs().try_into().unwrap(),
            is_private: data.is_private,
            owner: user.get_username(),
        })
    }

    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }

    pub fn is_private(&self) -> bool {
        self.is_private
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name_unchecked(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description_unchecked(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_color_unchecked(&mut self, color: String) {
        self.color = color;
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

#[derive(Debug, Queryable, Identifiable, Insertable, Selectable, Associations, Serialize)]
#[diesel(belongs_to(User, foreign_key = user))]
#[diesel(belongs_to(Room, foreign_key = room))]
#[diesel(primary_key(user, room))]
#[diesel(table_name = crate::schema::memberships)]
pub struct Membership {
    user: String,
    room: String,
    date_joined: i64,
}

impl Membership {
    pub fn new(user: &User, room: &Room) -> Self {
        Self {
            user: user.get_username(),
            room: room.get_id(),
            date_joined: time::UNIX_EPOCH.elapsed().unwrap().as_secs().try_into().unwrap(),
        }
    }
}

#[derive(Debug, Queryable, Insertable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = user))]
#[diesel(belongs_to(Room, foreign_key = room))]
#[diesel(primary_key(user, room))]
#[diesel(table_name = crate::schema::bans)]
pub struct Ban {
    user: String,
    room: String
}

impl Ban {
    pub fn new(user: &User, room: &Room) -> Self {
        Self {
            user: user.get_username(),
            room: room.get_id(),
        }
    }
}


#[derive(Debug, Queryable, Insertable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = user))]
#[diesel(belongs_to(Post, foreign_key = post))]
#[diesel(primary_key(user, post))]
#[diesel(table_name = crate::schema::bookmarks)]
pub struct Bookmark {
    user: String,
    post: String,
}

impl Bookmark {

    pub fn new(user: &User, post: &Post) -> Self {
        Self {
            user: user.get_username(),
            post: post.get_id(),
        }
    }

}


#[derive(Debug, Queryable, Insertable, Selectable, Identifiable)]
#[diesel(primary_key(follower, followed))]
#[diesel(table_name = crate::schema::follows)]
pub struct Following {
    follower: String,
    followed: String,
}

impl Following {
    pub fn new(follower: &User, followed: &User) -> Self {
        Self {
            follower: follower.get_username(),
            followed: followed.get_username(),
        }
    }
}


#[derive(Debug, Queryable, Insertable, Selectable, Identifiable, Associations, Serialize)]
#[diesel(primary_key(id, user))]
#[diesel(belongs_to(User, foreign_key = user))]
#[diesel(table_name = crate::schema::notifications)]
pub struct Notification {
    id: String,
    user: String,
    message: String,
    href: String,
    timestamp: i64,
}

impl Notification {
    pub async fn push(notification_type: String, emitter: &User, user: &User, message: String, href: String, connection: DBConnection) -> Result<(), warp::Rejection> {
        Self::push_unchecked(notification_type, emitter, user.get_username(), message, href, connection).await
    }

    pub async fn push_unchecked(notification_type: String, emitter: &User, username: String, message: String, href: String, connection: DBConnection) -> Result<(), warp::Rejection> {

        let timestamp = time::UNIX_EPOCH.elapsed().unwrap().as_secs().try_into().unwrap();

        match notification_timeouts::table
            .find((&notification_type, emitter.borrow_username(), &username))
            .first::<NotificationTimeout>(connection.lock().await.deref_mut()) {
                Ok(mut t) => {
                    if timestamp - t.timestamp_emitted < 3600 {
                        Err(CooldownActiveError)?;
                    } else {
                        t.renew(timestamp);
                        let _: Result<NotificationTimeout, _> = t.save_changes(connection.lock().await.deref_mut());
                    }
                },
                Err(diesel::NotFound) => {
                    
                },
                Err(_) => {
                    Err(InternalServerError)?;
                },
            }

        let _ = diesel::replace_into(notification_timeouts::table)
            .values(NotificationTimeout {
                notification_type,
                emitter: emitter.get_username(),
                receiver: username.clone(),
                timestamp_emitted: timestamp
            })
            .execute(connection.lock().await.deref_mut());

        let id = Uuid::new_v4().into();
        
        let notification = Self {
            id,
            user: username,
            message,
            href,
            timestamp,
        };

        diesel::insert_into(notifications::table)
            .values(notification)
            .execute(connection.lock().await.deref_mut())
            .map_err(|_| InternalServerError)?;
        
        Ok(())
    }
}


#[derive(Debug, Queryable, Insertable, Selectable, Identifiable, Serialize, AsChangeset)]
#[diesel(primary_key(notification_type, emitter, receiver))]
#[diesel(table_name = crate::schema::notification_timeouts)]
pub struct NotificationTimeout {
    notification_type: String,
    emitter: String,
    receiver: String,
    timestamp_emitted: i64,
}

impl NotificationTimeout {

    pub fn renew(&mut self, new_timestamp: i64) {
        self.timestamp_emitted = new_timestamp;
    }

}