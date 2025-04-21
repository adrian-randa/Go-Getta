-- Your SQL goes here
CREATE TABLE memberships (
    user VARCHAR(24) NOT NULL REFERENCES users(id),
    room VARCHAR(32) NOT NULL REFERENCES rooms(id),
    date_joined BIGINT NOT NULL,

    CONSTRAINT membership_constraint,
    PRIMARY KEY(user, room)
)