-- Add migration script here
ALTER TABLE users
   RENAME COLUMN id TO user_id;
