-- Your SQL goes here
CREATE TABLE posts (
    id VARCHAR(32) NOT NULL PRIMARY KEY,
    creator VARCHAR(24) NOT NULL REFERENCES users(username),
    body VARCHAR(300) NOT NULL,
    timestamp BIGINT NOT NULL,
    rating INTEGER NOT NULL,
    appendage_id VARCHAR(32),
    room VARCHAR(32) REFERENCES rooms(id),
    parent VARCHAR(32),
    comments INTEGER NOT NULL,
    shares INTEGER NOT NULL,
    reposts INTEGER NOT NULL,
    bookmarks INTEGER NOT NULL
)