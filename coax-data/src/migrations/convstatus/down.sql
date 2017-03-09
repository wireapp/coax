-- Remove status column from conversations.

CREATE TABLE IF NOT EXISTS conversations_tmp (
    id      BLOB PRIMARY KEY,
    name    TEXT,
    ctype   INTEGER NOT NULL,
    creator BLOB NOT NULL,
    muted   BOOLEAN NOT NULL,
    time    BIGINT NOT NULL,
    FOREIGN KEY (creator) REFERENCES users ON DELETE RESTRICT
) WITHOUT ROWID;

INSERT INTO conversations_tmp (
    id,
    name,
    ctype,
    creator,
    muted,
    time,
) SELECT id, name, ctype, creator, muted, time FROM conversations;

DROP TABLE conversations;

ALTER TABLE conversations_tmp RENAME TO conversations;

PRAGMA foreign_key_check(conversations);
