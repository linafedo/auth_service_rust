create table "tokens" (
    token text PRIMARY KEY NOT NULL,
    user_id uuid NOT NULL,
    created_at timestamp NULL,
    last_used_at timestamp NULL
);