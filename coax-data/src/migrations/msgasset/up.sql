-- Add asset column to messages.

ALTER TABLE messages ADD COLUMN asset TEXT REFERENCES assets;

