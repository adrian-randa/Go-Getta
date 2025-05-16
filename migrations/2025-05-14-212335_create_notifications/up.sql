-- Your SQL goes here
CREATE TABLE notifications (
    id VARCHAR(32) NOT NULL,
    user VARCHAR(24) NOT NULL REFERENCES users(username),
    message VARCHAR(150) NOT NULL,
    href VARCHAR(100) NOT NULL,
    timestamp BIGINT NOT NULL,

    CONSTRAINT notification_constraint,
    PRIMARY KEY(id, user)
)