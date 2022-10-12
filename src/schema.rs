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
    users (id) {
        password -> Varchar,
        login -> Text,
        id -> Uuid,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
