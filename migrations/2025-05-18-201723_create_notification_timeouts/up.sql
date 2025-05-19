-- Your SQL goes here
CREATE TABLE notification_timeouts (
    notification_type VARCHAR(16) NOT NULL,
    emitter VARCHAR(24) NOT NULL REFERENCES users(username),
    receiver VARCHAR(24) NOT NULL REFERENCES users(username),
    timestamp_emitted BIGINT NOT NULL,

    CONSTRAINT notification_timeout_constraint,
    PRIMARY KEY(notification_type, emitter, receiver)
)