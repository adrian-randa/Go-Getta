-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN followers;

ALTER TABLE users
DROP COLUMN followed;