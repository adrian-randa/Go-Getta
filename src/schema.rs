// @generated automatically by Diesel CLI.

diesel::table! {
    account_keys (key) {
        key -> Text,
        used -> Bool,
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
    }
}

diesel::joinable!(posts -> rooms (room));
diesel::joinable!(posts -> users (creator));
diesel::joinable!(sessions -> users (username));

diesel::allow_tables_to_appear_in_same_query!(
    account_keys,
    posts,
    rooms,
    sessions,
    users,
);
