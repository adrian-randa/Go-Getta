-- Your SQL goes here
CREATE TABLE bans (
    user VARCHAR(24) NOT NULL REFERENCES users(username),
    room VARCHAR(32) NOT NULL REFERENCES rooms(id),

    CONSTRAINT ban_constraint,
    PRIMARY KEY (user, room)
)