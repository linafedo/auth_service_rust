-- Create Users Table
CREATE TABLE users(
    id       uuid NOT NULL,
    PRIMARY KEY (id),
    login    TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);
