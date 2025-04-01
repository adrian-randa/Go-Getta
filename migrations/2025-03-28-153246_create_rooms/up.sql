-- Your SQL goes here
CREATE TABLE rooms (
    id VARCHAR(32) NOT NULL PRIMARY KEY,
    name VARCHAR(24) NOT NULL,
    description VARCHAR(150) NOT NULL,
    color VARCHAR(6) NOT NULL,
    date_created BIGINT NOT NULL,
    is_private BOOLEAN NOT NULL
)