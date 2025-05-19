#![recursion_limit = "256"]

use diesel::{associations::HasTable, prelude::*};
use dotenvy::dotenv;
use tokio::select;
use warp::Filter;
use std::{collections::HashMap, env, fs, ops::DerefMut, sync::Arc};
use uuid::*;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use go_getta::{api::{bookmark::{bookmark_post, fetch_bookmarked_posts, unbookmark_post}, file_upload::{file_upload, update_profile_picture, update_room_banner}, follow::{fetch_followed, fetch_followed_feed, fetch_followers, follow, is_following, unfollow}, notification::{delete_notifications, fetch_notifications}, post::{create_post, delete_post, get_post, register_post_share}, public_space::public_space_query, rating::set_rating_state, room::{add_user_to_room, ban_user_from_room, create_room, delete_room, fetch_banned_users, fetch_joined_users, get_joined_rooms, join_room, kick_user_from_room, leave_room, room_posts_query, search_for_banned_user, search_for_room_member, unban_user_from_room, update_room_color, update_room_description, update_room_name}, search::{fetch_search_posts, fetch_search_rooms, fetch_search_users}, thread::{comment_query, get_thread}, user_data::{get_user_data, update_biography, update_public_name, users_posts_query}, who_am_i}, clean_database, create_account::create_account, db::{establish_connection, scan_for_keys, with_db_connection}, login::*, models::*, pages::{with_page_store, PageStore}, render::render, schema::sessions::{self, timestamp}, session_gate};


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

macro_rules! combine_routes {
    ($a:ident $(, $b:ident)+ $(,)?) => {
        $a.boxed()$(.or($b.boxed()))+
    };
}

#[tokio::main]
async fn main() {
    let connection = establish_connection();
    connection.lock().await.run_pending_migrations(MIGRATIONS).unwrap();

    scan_for_keys(connection.clone()).await;
    clean_database(connection.clone()).await;

    let page_store = PageStore::init();

    let public_route = warp::get()
        .and(warp::any())
        .and(warp::fs::dir("./public"));

    let main_route = warp::get()
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_page_store(page_store.clone()))
        .and(with_db_connection(connection.clone()))
        .and_then(render);

    let login_route = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db_connection(connection.clone()))
        .and_then(login);

    let logout_route = warp::delete()
        .and(warp::path("logout"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and_then(logout);

    let create_account_route = warp::post()
        .and(warp::path("create_account"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db_connection(connection.clone()))
        .and_then(create_account);

    let who_am_i_route = warp::get()
        .and(warp::path("api"))
        .and(warp::path("who_am_i"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and_then(who_am_i);

    let create_post_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("create_post"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(create_post);

    let delete_post_route = warp::delete()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "delete_post" / String))
        .and_then(delete_post);

    let bookmark_post_route = warp::post()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "bookmark_post" / String))
        .and_then(bookmark_post);

    let unbookmark_post_route = warp::post()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "unbookmark_post" / String))
        .and_then(unbookmark_post);

    let fetch_bookmarked_posts_route = warp::get()
        .and(warp::path("api"))
        .and(warp::path("fetch_bookmarked_posts"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(fetch_bookmarked_posts);

    let public_space_route = warp::get()
        .and(warp::path("api"))
        .and(warp::path("fetch_public_space"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(public_space_query);

    let file_upload_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("file_upload"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(file_upload);

    let update_profile_picture_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("update_profile_picture"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::multipart::form().max_length(2_000_000))
        .and_then(update_profile_picture);

    let users_posts_query = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path!("api" / "users_posts" / String))
        .and_then(users_posts_query);

    let get_user_data_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "get_user_data" / String))
        .and_then(get_user_data);

    let get_post_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "get_post" / String))
        .and_then(get_post);

    let register_post_share_route = warp::post()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "register_post_share" / String))
        .and_then(register_post_share);

    let set_rating_state_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("set_rating_state"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(set_rating_state);

    let get_thread_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "get_thread" / String))
        .and_then(get_thread);

    let fetch_comments_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "fetch_comments" / String))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(comment_query);

    let update_public_name_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("update_public_name"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(update_public_name);

    let update_biography_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("update_biography"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(update_biography);

    let create_room_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("create_room"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(create_room);

    let update_room_banner_route = warp::post()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "update_room_banner" / String))
        .and(warp::multipart::form().max_length(3_000_000))
        .and_then(update_room_banner);

    let update_room_name_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("update_room_name"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(update_room_name);

    let update_room_description_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("update_room_description"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(update_room_description);

    let update_room_color_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("update_room_color"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(update_room_color);

    let delete_room_route = warp::delete()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "delete_room" / String))
        .and_then(delete_room);

    let leave_room_route = warp::delete()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "leave_room" / String))
        .and_then(leave_room);

    let join_room_route = warp::post()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "join_room" / String))
        .and_then(join_room);

    let get_joined_rooms_route = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get_joined_rooms"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and_then(get_joined_rooms);

    let fetch_room_posts_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "fetch_room_posts" / String))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(room_posts_query);

    let fetch_room_members_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path!("api" / "fetch_joined_users" / String))
        .and_then(fetch_joined_users);

    let search_for_room_member_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path!("api" / "search_for_room_member" / String))
        .and_then(search_for_room_member);

    let add_user_to_room_route = warp::post()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "add_user_to_room" / String))
        .and(warp::body::json())
        .and_then(add_user_to_room);

    let kick_user_from_room_route = warp::delete()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "kick_user_from_room" / String / String))
        .and_then(kick_user_from_room);

    let ban_user_from_room_route = warp::delete()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "ban_user_from_room" / String / String))
        .and_then(ban_user_from_room);

    let unban_user_from_room_route = warp::post()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "unban_user_from_room" / String / String))
        .and_then(unban_user_from_room);

    let fetch_banned_users_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path!("api" / "fetch_banned_users" / String))
        .and_then(fetch_banned_users);

    let search_for_banned_user_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path!("api" / "search_for_banned_user" / String))
        .and_then(search_for_banned_user);

    let search_posts_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path!("api" / "search" / "posts"))
        .and_then(fetch_search_posts);

    let search_rooms_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path!("api" / "search" / "rooms"))
        .and_then(fetch_search_rooms);

    let search_users_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path!("api" / "search" / "users"))
        .and_then(fetch_search_users);

    let storage_route = warp::get()
        .and(warp::path("storage"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and_then(session_gate)
        .and(warp::fs::dir(env::var("STORAGE_URL").unwrap()))
        .map(|_, file| file);

    let follow_route = warp::post()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "follow" / String))
        .and_then(follow);

    let unfollow_route = warp::delete()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "unfollow" / String))
        .and_then(unfollow);

    let is_following_route = warp::get()
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::path!("api" / "is_following" / String))
        .and_then(is_following);

    let fetch_followed_feed_route = warp::get()
        .and(warp::path!("api" / "fetch_followed_feed"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(fetch_followed_feed);

    let fetch_followers_route = warp::get()
        .and(warp::path!("api" / "fetch_followers"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(fetch_followers);

    let fetch_followed_route = warp::get()
        .and(warp::path!("api" / "fetch_followed"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(fetch_followed);

    let fetch_notifications_route = warp::get()
        .and(warp::path!("api" / "fetch_notifications"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(fetch_notifications);

    let delete_notifications_route = warp::delete()
        .and(warp::path!("api" / "delete_notifications"))
        .and(warp::header::headers_cloned())
        .and(with_db_connection(connection.clone()))
        .and(warp::body::json())
        .and_then(delete_notifications);


    let routes = combine_routes!(
        public_route,
        main_route,
        login_route,
        logout_route,
        create_account_route,
        who_am_i_route,
        create_post_route,
        delete_post_route,
        public_space_route,
        file_upload_route,
        update_profile_picture_route,
        get_user_data_route,
        users_posts_query,
        set_rating_state_route,
        register_post_share_route,
        get_thread_route,
        fetch_comments_route,
        update_public_name_route,
        update_biography_route,
        get_post_route,
        create_room_route,
        update_room_banner_route,
        update_room_name_route,
        update_room_description_route,
        update_room_color_route,
        delete_room_route,
        leave_room_route,
        join_room_route,
        get_joined_rooms_route,
        fetch_room_posts_route,
        fetch_room_members_route,
        search_for_room_member_route,
        add_user_to_room_route,
        kick_user_from_room_route,
        ban_user_from_room_route,
        unban_user_from_room_route,
        fetch_banned_users_route,
        search_for_banned_user_route,
        bookmark_post_route,
        unbookmark_post_route,
        fetch_bookmarked_posts_route,
        search_posts_route,
        search_rooms_route,
        search_users_route,
        follow_route,
        unfollow_route,
        is_following_route,
        storage_route,
        fetch_followed_feed_route,
        fetch_followed_route,
        fetch_followers_route,
        fetch_notifications_route,
        delete_notifications_route,
    );

    select! {
        _ = warp::serve(routes).run(([0, 0, 0, 0], 7500)) => (),
        _ = tokio::signal::ctrl_c() => {},
    }
}

