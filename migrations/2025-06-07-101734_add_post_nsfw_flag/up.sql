-- Your SQL goes here
ALTER TABLE posts
ADD COLUMN is_nsfw BOOLEAN NOT NULL DEFAULT false;