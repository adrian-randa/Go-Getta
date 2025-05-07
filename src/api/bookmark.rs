use std::{collections::HashMap, ops::DerefMut};

use crate::{db::DBConnection, error::{InternalServerError, InvalidQueryError, InvalidSessionError, PostDoesNotExistError}, models::{Bookmark, Post}, schema::{bookmarks, posts}, validate_session_from_headers};

use diesel::{result::Error::NotFound, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SaveChangesDsl};

use super::PostQueryResponse;


pub async fn bookmark_post(headers: warp::http::HeaderMap, connection: DBConnection, post_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let mut post: Post = posts::table
        .find(&post_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| PostDoesNotExistError)?;

    let username = user.get_username();

    let record = bookmarks::table
        .find((&username, &post_id))
        .first::<Bookmark>(connection.lock().await.deref_mut());
    
    match record {
        Ok(_) => {}
        Err(NotFound) => {
            connection.lock().await.exclusive_transaction(|conn| {
                diesel::insert_into(bookmarks::table)
                    .values(&Bookmark::new(&user, &post))
                    .execute(conn)?;

                post.set_bookmarks_unchecked(post.get_bookmarks() + 1);
                let _: Post = post.save_changes(conn)?;

                Ok(())
            }).map_err(|_: diesel::result::Error| InternalServerError)?;
        }
        _ => {
            Err(InternalServerError)?;
        }
    }

    Ok(warp::reply::json(&PostQueryResponse::from_post_for_user(post, &user, connection.clone()).await?))
}

pub async fn unbookmark_post(headers: warp::http::HeaderMap, connection: DBConnection, post_id: String) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let username = user.get_username();

    let mut post: Post = posts::table
        .find(&post_id)
        .first(connection.lock().await.deref_mut())
        .map_err(|_| PostDoesNotExistError)?;

    let record = bookmarks::table
        .find((&username, &post_id))
        .first::<Bookmark>(connection.lock().await.deref_mut());
    match record {
        Ok(_) => {
            connection.lock().await.exclusive_transaction(|conn| {
                diesel::delete(bookmarks::table.find((&username, &post_id)))
                    .execute(conn)?;

                post.set_bookmarks_unchecked(post.get_bookmarks() - 1);
                let _: Post = post.save_changes(conn)?;

                Ok(())
            }).map_err(|_: diesel::result::Error| InternalServerError)?;
        }
        Err(NotFound) => {Err(InvalidQueryError)?}
        _ => {Err(InternalServerError)?}
    }

    Ok(warp::reply::json(&PostQueryResponse::from_post_for_user(post, &user, connection).await?))
}

pub async fn fetch_bookmarked_posts(headers: warp::http::HeaderMap, connection: DBConnection, query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;
    let username = user.get_username();

    let page = query.get("page").ok_or(InvalidQueryError)?.parse::<i64>().map_err(|_| InvalidQueryError)?;

    let posts: Vec<_> = bookmarks::table
        .filter(bookmarks::user.eq(&username))
        .offset(20 * page)
        .limit(20)
        .inner_join(posts::table)
        .load_iter(connection.lock().await.deref_mut())
        .map_err(|_| InternalServerError)?
        .filter_map(|r: QueryResult<(Bookmark, Post)>| {
            let (_, p) = r.ok()?;
            Some(p)
        })
        .collect();

    let mut response = Vec::with_capacity(posts.len());

    for post in posts {
        response.push(PostQueryResponse::from_post_for_user(post, &user, connection.clone()).await?);
    }

    Ok(warp::reply::json(&response))
}