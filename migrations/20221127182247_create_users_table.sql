-- Add migration script here
CREATE TABLE users(
    id       uuid NOT NULL,
    login    TEXT NOT NULL,
    password TEXT NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (login)
);