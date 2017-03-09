-- Add user_id column to messages.

ALTER TABLE messages ADD COLUMN user_id BLOB REFERENCES users ON DELETE RESTRICT;

