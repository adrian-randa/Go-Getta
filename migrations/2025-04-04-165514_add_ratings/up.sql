-- Your SQL goes here
CREATE TABLE ratings (
    user VARCHAR(24) NOT NULL REFERENCES users(username),
    post VARCHAR(32) NOT NULL REFERENCES posts(id),
    is_upvote BOOLEAN NOT NULL,

    CONSTRAINT rating_relation,
    PRIMARY KEY(user, post)
)