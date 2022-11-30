-- Add migration script here
CREATE TABLE users (
    id            uuid NOT NULL,
    login         TEXT NOT NULL,
    salt          TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (login)
);