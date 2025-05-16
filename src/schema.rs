// @generated automatically by Diesel CLI.

diesel::table! {
    account_keys (key) {
        key -> Text,
        used -> Bool,
    }
}

diesel::table! {
    bans (user, room) {
        user -> Text,
        room -> Text,
    }
}

diesel::table! {
    bookmarks (user, post) {
        user -> Text,
        post -> Text,
    }
}

diesel::table! {
    follows (follower, followed) {
        follower -> Text,
        followed -> Text,
    }
}

diesel::table! {
    memberships (user, room) {
        user -> Text,
        room -> Text,
        date_joined -> BigInt,
    }
}

diesel::table! {
    notifications (id, user) {
        id -> Text,
        user -> Text,
        message -> Text,
        href -> Text,
        timestamp -> BigInt,
    }
}

diesel::table! {
    posts (id) {
        id -> Text,
        creator -> Text,
        body -> Text,
        timestamp -> BigInt,
        rating -> Integer,
        appendage_id -> Nullable<Text>,
        room -> Nullable<Text>,
        parent -> Nullable<Text>,
        comments -> Integer,
        shares -> Integer,
        reposts -> Integer,
        bookmarks -> Integer,
        child -> Nullable<Text>,
    }
}

diesel::table! {
    ratings (user, post) {
        user -> Text,
        post -> Text,
        is_upvote -> Bool,
    }
}

diesel::table! {
    rooms (id) {
        id -> Text,
        name -> Text,
        description -> Text,
        color -> Text,
        date_created -> BigInt,
        is_private -> Bool,
        owner -> Text,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        username -> Text,
        timestamp -> Nullable<BigInt>,
    }
}

diesel::table! {
    users (username) {
        username -> Text,
        password -> Text,
        public_name -> Text,
        biography -> Text,
        followers -> Integer,
        followed -> Integer,
    }
}

diesel::joinable!(bans -> rooms (room));
diesel::joinable!(bans -> users (user));
diesel::joinable!(bookmarks -> posts (post));
diesel::joinable!(bookmarks -> users (user));
diesel::joinable!(memberships -> rooms (room));
diesel::joinable!(memberships -> users (user));
diesel::joinable!(notifications -> users (user));
diesel::joinable!(posts -> rooms (room));
diesel::joinable!(posts -> users (creator));
diesel::joinable!(ratings -> posts (post));
diesel::joinable!(ratings -> users (user));
diesel::joinable!(rooms -> users (owner));
diesel::joinable!(sessions -> users (username));

diesel::allow_tables_to_appear_in_same_query!(
    account_keys,
    bans,
    bookmarks,
    follows,
    memberships,
    notifications,
    posts,
    ratings,
    rooms,
    sessions,
    users,
);
