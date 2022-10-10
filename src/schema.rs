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
        password -> Varchar,
        user_login -> Text,
        user_id -> Uuid,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
