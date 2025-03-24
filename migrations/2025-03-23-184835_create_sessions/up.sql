-- Your SQL goes here
CREATE TABLE sessions (
    id VARCHAR(32) NOT NULL PRIMARY KEY,
    username VARCHAR(24) NOT NULL REFERENCES users(username),
    timestamp INTEGER -- Null if session should never expire
)