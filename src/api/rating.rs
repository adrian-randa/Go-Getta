use diesel::{QueryDsl, RunQueryDsl, SaveChangesDsl};
use serde::Deserialize;

use crate::{db::DBConnection, error::*, models::{Post, Rating}, schema::{posts, ratings}, validate_session_from_headers};

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

    Ok(warp::reply::json(&PostQueryResponse::from_post_for_user(new_post, &user, connection).await))
}