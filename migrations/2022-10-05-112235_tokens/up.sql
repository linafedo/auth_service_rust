create table "tokens" (
    token varchar PRIMARY KEY,
    user_id uuid NOT NULL,
    created_at timestamp NOT NULL,
    last_used_at timestamp NOT NULL
);