-- Your SQL goes here
CREATE TABLE follows (
    follower VARCHAR(24) NOT NULL REFERENCES users(username),
    followed VARCHAR(24) NOT NULL REFERENCES users(username),

    CONSTRAINT follow_constraint,
    PRIMARY KEY(follower, followed)
)