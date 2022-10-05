// @generated automatically by Diesel CLI.

diesel::table! {
    tokens (token) {
        token -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamp,
        last_used_at -> Timestamp,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        password -> Varchar,
        user_login -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
