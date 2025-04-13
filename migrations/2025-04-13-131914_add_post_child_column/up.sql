-- Your SQL goes here
ALTER TABLE posts
ADD child VARCHAR(32) REFERENCES posts(id)