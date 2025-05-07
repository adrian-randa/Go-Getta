use std::ops::DerefMut;

use diesel::{result::Error::NotFound, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::{db::{with_db_connection, DBConnection}, error::{InternalServerError, InvalidSessionError, PostDoesNotExistError}, models::{Bookmark, Post, Rating, User}, schema::{bookmarks, posts, ratings}, validate_session_from_headers};

pub mod post;
pub mod public_space;
pub mod file_upload;
pub mod user_data;
pub mod rating;
pub mod thread;
pub mod room;
pub mod bookmark;

#[derive(Debug, Serialize)]
struct WhoAmIResponse {
    username: String,
    public_name: String,
    biography: String,
}

impl WhoAmIResponse {
    fn from_user(user: User) -> Self {
        Self { username: user.get_username(), public_name: user.get_public_name(), biography: user.get_biography() }
    }
}

pub async fn who_am_i(headers: warp::http::HeaderMap, connection: DBConnection) -> Result<impl warp::Reply, warp::Rejection> {
    let user = validate_session_from_headers(&headers, connection).await.ok_or(InvalidSessionError)?;

    Ok(warp::reply::json(&WhoAmIResponse::from_user(user)))
}

#[derive(Debug, Serialize)]
pub struct PostQueryResponse {
    post: Post,
    interaction: PostInteraction,
    child: Option<Post>,
}

impl PostQueryResponse {
    pub async fn from_post_for_user(post: Post, user: &User, connection: DBConnection) -> Result<Self, warp::Rejection> {

        let username = user.get_username();
        let post_id = post.get_id();

        let rating: RatingInteraction = ratings::table
            .find((&username, &post_id))
            .first(connection.lock().await.deref_mut()).ok().into();

        let mut child: Option<Post> = None;
        if let Some(child_id) = post.get_child() {
            child = posts::table
                .find(child_id)
                .first(connection.lock().await.deref_mut())
                .ok();
        }

        let bookmarked = match bookmarks::table
            .find((&username, &post_id))
            .first::<Bookmark>(connection.lock().await.deref_mut()) {
                Ok(_) => true,
                Err(NotFound) => false,
                _ => {Err(InternalServerError)?}
            };


        Ok(Self {
            post,
            interaction: PostInteraction {
                rating,
                bookmarked
            },
            child
        })
    }

    pub fn get_post_ref(&self) -> &Post {
        &self.post
    }
}

#[derive(Debug, Serialize)]
pub struct PostInteraction {
    rating: RatingInteraction,
    bookmarked: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RatingInteraction {
    None,
    Upvote,
    Downvote,
}

impl From<Option<Rating>> for RatingInteraction {
    fn from(value: Option<Rating>) -> Self {
        if let Some(rating) = value {
            if rating.is_upvote() {
                return Self::Upvote
            } else {
                return Self::Downvote
            }
        }
        
        Self::None
    }
}

impl RatingInteraction {
    pub fn get_delta(&self, other: &Self) -> i32 {
        let old = match self {
            RatingInteraction::None => 0,
            RatingInteraction::Upvote => 1,
            RatingInteraction::Downvote => -1,
        };

        let new = match other {
            RatingInteraction::None => 0,
            RatingInteraction::Upvote => 1,
            RatingInteraction::Downvote => -1,
        };

        new - old
    }
}