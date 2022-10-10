create table "tokens" (
    token VARCHAR PRIMARY KEY,
    user_id uuid NOT NULL,
    created_at TIMESTAMP NOT NULL,
    last_used_at TIMESTAMP NOT NULL
);