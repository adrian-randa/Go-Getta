-- Your SQL goes here
CREATE TABLE users (
    username VARCHAR(24) NOT NULL PRIMARY KEY,
    password VARCHAR(32) NOT NULL,
    public_name VARCHAR(32) NOT NULL,
    biography VARCHAR(2048) NOT NULL
)