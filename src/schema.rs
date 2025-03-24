// @generated automatically by Diesel CLI.

diesel::table! {
    account_keys (key) {
        key -> Text,
        used -> Bool,
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

diesel::joinable!(sessions -> users (username));

diesel::allow_tables_to_appear_in_same_query!(
    account_keys,
    sessions,
    users,
);
