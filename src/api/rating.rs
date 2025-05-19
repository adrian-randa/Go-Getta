use diesel::{QueryDsl, RunQueryDsl, SaveChangesDsl};
use serde::Deserialize;

use crate::{db::DBConnection, error::*, models::{Notification, Post, Rating, User}, schema::{posts, ratings}, validate_session_from_headers};

use super::{PostQueryResponse, RatingInteraction};

#[derive(Debug, Deserialize)]
pub struct SetRatingData {
    post_id: String,
    new_rating: RatingInteraction,
}

pub async fn set_rating_state(headers: warp::http::HeaderMap, connection: DBConnection, data: SetRatingData) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;
    
    let new_post: Post = connection.lock().await.exclusive_transaction(|conn| {

        let mut post: Post = posts::table
            .find(data.post_id).first(conn)?;

        let old_rating: RatingInteraction = ratings::table
            .find((user.get_username(), post.get_id()))
            .first(conn).ok().into();

        if data.new_rating == RatingInteraction::None {
            let _ = diesel::delete(ratings::table.find((user.get_username(), post.get_id()))).execute(conn);
        } else {
            diesel::replace_into(ratings::table)
                .values(&Rating::new(&user, &post, data.new_rating == RatingInteraction::Upvote))
                .execute(conn)?;
        }

        post.set_rating_unchecked(post.get_rating() + old_rating.get_delta(&data.new_rating));

        let _: Post = post.save_changes(conn)?;

        Ok(post)
    }).map_err(|_: diesel::result::Error| InternalServerError)?;

    let post_creator = new_post.get_creator();
    let post_id = new_post.get_id();

    let response = PostQueryResponse::from_post_for_user(new_post, &user, connection.clone()).await?;

    if user.borrow_username() != &post_creator {
        tokio::spawn(rating_state_notification_rollout(user, post_creator, post_id, data.new_rating, connection));
    }

    Ok(warp::reply::json(&response))
}

async fn rating_state_notification_rollout(emitter: User, receiver: String, post_id: String, rating: RatingInteraction, connection: DBConnection) {
    if rating == RatingInteraction::Upvote {
        let _ = Notification::push_unchecked(
            "Upvote".into(),
            &emitter,
            receiver,
            format!("{} liked your post!", emitter.get_public_name()),
            format!("?view=post&id={}", post_id),
            connection
        ).await;
    }
}