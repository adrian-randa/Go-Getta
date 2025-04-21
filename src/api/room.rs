use std::ops::DerefMut;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use warp::filters::multipart::FormData;

use crate::{db::DBConnection, error::{InternalServerError, InvalidSessionError}, models::{Membership, Room}, schema::{memberships::{self}, rooms}, validate_session_from_headers};

#[derive(Debug, Deserialize)]
pub struct RoomCreationData {
    pub name: String,
    pub description: String,
    pub color: String,
    pub is_private: bool
}

#[derive(Debug, Serialize)]
struct RoomCreationResponse {
    room_id: String,
}

pub async fn create_room(headers: warp::http::HeaderMap, connection: DBConnection, data: RoomCreationData) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let room = Room::try_open(data, &user)?;
    let membership = Membership::new(&user, &room);

    let room_id = room.get_id();

    connection.lock().await.exclusive_transaction(|conn| {
        diesel::insert_into(rooms::table)
            .values(room)
            .execute(conn)?;

        diesel::insert_into(memberships::table)
            .values(membership)
            .execute(conn)?;

        diesel::result::QueryResult::Ok(())
    }).map_err(|_| InternalServerError)?;

    Ok(warp::reply::json(&RoomCreationResponse { room_id }))
}

pub async fn get_joined_rooms(headers: warp::http::HeaderMap, connection: DBConnection) -> Result<impl warp::Reply, warp::Rejection> {
    use memberships::user;

    let member_user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let member_username = member_user.get_username();

    let membership_associations: Vec<Room> = memberships::table
        .filter(user.eq(member_username))
        .inner_join(rooms::table)
        .select(Room::as_select())
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    Ok(warp::reply::json(&membership_associations))
}