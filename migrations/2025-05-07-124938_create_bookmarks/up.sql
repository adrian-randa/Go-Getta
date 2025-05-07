-- Your SQL goes here
CREATE TABLE bookmarks (
    user VARCHAR(24) NOT NULL REFERENCES users(username),
    post VARCHAR(32) NOT NULL REFERENCES posts(id),

    CONSTRAINT bookmark_constraint,
    PRIMARY KEY(user, post)
)