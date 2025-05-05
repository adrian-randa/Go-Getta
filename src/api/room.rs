use std::{collections::HashMap, ops::{Deref, DerefMut}};

use diesel::{AsChangeset, BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SaveChangesDsl, SelectableHelper, TextExpressionMethods};
use serde::{Deserialize, Serialize};
use warp::filters::multipart::FormData;

use crate::{api::PostQueryResponse, db::DBConnection, error::{ContentTooLargeError, EmptyContentError, InsufficientPermissionsError, InternalServerError, InvalidQueryError, InvalidSessionError, RoomBoundaryViolationError, RoomDoesNotExistError, UserDoesNotExistError, UserIsBannedError}, models::{Ban, Membership, Post, Room, User}, schema::{bans, memberships::{self}, posts::{self, timestamp}, rooms, users}, validate_session_from_headers};

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

pub async fn room_posts_query(headers: warp::http::HeaderMap, connection: DBConnection, room_id: String, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let _membership: Membership = memberships::table
        .find((user.get_username(), &room_id))
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomBoundaryViolationError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;

    use posts::room;
    let posts: Vec<Post> = posts::table
        .filter(room.eq(&room_id))
        .order(timestamp.desc())
        .offset(page * 20)
        .limit(20)
        .load(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    let mut response = Vec::with_capacity(posts.len());

    for post in posts {
        response.push(PostQueryResponse::from_post_for_user(post, &user, connection.clone()).await);
    }

    Ok(warp::reply::json(&response))
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoomNameData {
    room_id: String,
    new_name: String,
}

pub async fn update_room_name(headers: warp::http::HeaderMap, connection: DBConnection, data: UpdateRoomNameData) -> Result<impl warp::Reply, warp::Rejection> {

    if data.new_name.len() > 24 {
        Err(ContentTooLargeError)?;
    }

    if data.new_name.len() == 0 {
        Err(EmptyContentError)?;
    }

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let mut room: Room = rooms::table
        .find(data.room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    room.set_name_unchecked(data.new_name);
    let _: Room = room.save_changes(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}


#[derive(Debug, Deserialize)]
pub struct UpdateRoomDescriptionData {
    room_id: String,
    new_description: String,
}

pub async fn update_room_description(headers: warp::http::HeaderMap, connection: DBConnection, data: UpdateRoomDescriptionData) -> Result<impl warp::Reply, warp::Rejection> {

    if data.new_description.len() > 150 {
        Err(ContentTooLargeError)?;
    }

    if data.new_description.len() == 0 {
        Err(EmptyContentError)?;
    }

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let mut room: Room = rooms::table
        .find(data.room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    room.set_description_unchecked(data.new_description);
    let _: Room = room.save_changes(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?; 

    Ok(warp::reply())
}


#[derive(Debug, Deserialize)]
pub struct UpdateRoomColorData {
    room_id: String,
    new_color: String,
}

pub async fn update_room_color(headers: warp::http::HeaderMap, connection: DBConnection, data: UpdateRoomColorData) -> Result<impl warp::Reply, warp::Rejection> {

    if data.new_color.len() != 6 {
        Err(InvalidQueryError)?;
    }

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let mut room: Room = rooms::table
        .find(data.room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    room.set_color_unchecked(data.new_color);
    let _: Room = room.save_changes(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}


pub async fn delete_room(headers: warp::http::HeaderMap, connection: DBConnection, room_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let room: Room = rooms::table
        .find(room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    let _ = diesel::delete(rooms::table.find(room.get_id()))
        .execute(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    tokio::spawn(room_deletion_cleanup(connection, room));

    Ok(warp::reply())
}

async fn room_deletion_cleanup(connection: DBConnection, room: Room) {
    let room_id = room.get_id();
    
    // Remove Memberships
    let _ = diesel::delete(
        memberships::table.filter(memberships::room.eq(&room_id))
    ).execute(connection.lock().await.deref_mut());


    // Remove Posts
    let _ = diesel::delete(
        posts::table.filter(posts::room.eq(&room_id))
    ).execute(connection.lock().await.deref_mut());
}


pub async fn leave_room(headers: warp::http::HeaderMap, connection: DBConnection, room_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let room: Room = rooms::table
        .find(&room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    let username = user.get_username();

    if room.get_owner() == username {
        Err(InvalidQueryError)?;
    }

    diesel::delete(memberships::table.find((username, room_id)))
        .execute(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}


#[derive(Debug, Serialize)]
struct JoinedUserResponse {
    username: String,
    public_name: String,
}

pub async fn fetch_joined_users(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>, room_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let _user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;
    
    use crate::schema::memberships::room;
    let response: Vec<_> = memberships::table
        .filter(room.eq(&room_id))
        .offset(20 * page)
        .limit(20)
        .inner_join(users::table)
        .load_iter(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?
        .filter_map(|r: QueryResult<(Membership, User)>| {
            let (_, u) = r.ok()?;
            Some(JoinedUserResponse { username: u.get_username(), public_name: u.get_public_name() })
        })
        .collect();

    Ok(warp::reply::json(&response))
}

pub async fn search_for_room_member(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>, room_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let _user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let search_term = query.get("query").ok_or(InvalidQueryError)?;

    use crate::schema::memberships::room;
    let matches: Vec<_> = memberships::table
        .filter(room.eq(room_id).and(memberships::user.like(format!("%{}%", search_term))))
        .inner_join(users::table)
        .load_iter(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?
        .filter_map(|r: QueryResult<(Membership, User)>| {
            let (_, u) = r.ok()?;
            Some(JoinedUserResponse { username: u.get_username(), public_name: u.get_public_name() })
        })
        .collect();


    Ok(warp::reply::json(&matches))
}

pub async fn kick_user_from_room(headers: warp::http::HeaderMap, connection: DBConnection, room_id: String, username: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    if user.get_username() == username {
        Err(InvalidQueryError)?;
    }

    let room: Room = rooms::table
        .find(&room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    diesel::delete(
        memberships::table.find((username, room_id))
    ).execute(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}

pub async fn ban_user_from_room(headers: warp::http::HeaderMap, connection: DBConnection, room_id: String, username: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    if user.get_username() == username {
        Err(InvalidQueryError)?;
    }

    let room: Room = rooms::table
        .find(&room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    let banned_user: User = users::table
        .find(&username)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| UserDoesNotExistError)?;

    diesel::delete(
        memberships::table.find((username, room_id))
    ).execute(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    diesel::replace_into(bans::table).values(
        Ban::new(&banned_user, &room)
    ).execute(connection.lock().await.deref_mut()).map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}

pub async fn unban_user_from_room(headers: warp::http::HeaderMap, connection: DBConnection, room_id: String, username: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let room: Room = rooms::table
        .find(&room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    diesel::delete(bans::table.find((&username, &room_id)))
        .execute(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}

pub async fn fetch_banned_users(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>, room_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let room: Room = rooms::table
        .find(&room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;
    
    let response: Vec<_> = bans::table
        .filter(bans::room.eq(&room_id))
        .offset(20 * page)
        .limit(20)
        .inner_join(users::table)
        .load_iter(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?
        .filter_map(|r: QueryResult<(Ban, User)>| {
            let (_, u) = r.ok()?;
            Some(JoinedUserResponse { username: u.get_username(), public_name: u.get_public_name() })
        })
        .collect();

    Ok(warp::reply::json(&response))
}

pub async fn search_for_banned_user(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>, room_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let room: Room = rooms::table
        .find(&room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    let search_term = query.get("query").ok_or(InvalidQueryError)?;

    let matches: Vec<_> = bans::table
        .filter(bans::room.eq(room_id).and(bans::user.like(format!("%{}%", search_term))))
        .inner_join(users::table)
        .load_iter(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?
        .filter_map(|r: QueryResult<(Ban, User)>| {
            let (_, u) = r.ok()?;
            Some(JoinedUserResponse { username: u.get_username(), public_name: u.get_public_name() })
        })
        .collect();

    Ok(warp::reply::json(&matches))
}

#[derive(Debug, Deserialize)]
pub struct AddUserToRoomData {
    username: String,
}

pub async fn add_user_to_room(headers: warp::http::HeaderMap, connection: DBConnection, room_id: String, data: AddUserToRoomData) -> Result<impl warp::Reply, warp::Rejection> {

    let username = data.username;

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let room: Room = rooms::table
        .find(&room_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| RoomDoesNotExistError)?;

    if room.get_owner() != user.get_username() {
        Err(InsufficientPermissionsError)?;
    }

    let ban_query: Result<Ban, _>=  bans::table.find((&username, &room_id)).first(connection.lock().await.deref_mut());

    if ban_query.is_ok() {
        Err(UserIsBannedError)?;
    }
    
    if ban_query.is_err_and(|e| e != diesel::NotFound) {
        Err(InternalServerError)?;
    }

    let added_user: User = users::table
        .find(&username)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| UserDoesNotExistError)?;

    diesel::replace_into(memberships::table)
        .values(Membership::new(&added_user, &room))
        .execute(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}