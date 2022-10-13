// @generated automatically by Diesel CLI.

diesel::table! {
    tokens (token) {
        token -> Text,
        user_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        last_used_at -> Nullable<Timestamp>,
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
