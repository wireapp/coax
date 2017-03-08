-- Add conversation status column.

CREATE TABLE IF NOT EXISTS conversations_tmp (
    id      BLOB PRIMARY KEY,
    name    TEXT,
    ctype   INTEGER NOT NULL,
    creator BLOB NOT NULL,
    muted   BOOLEAN NOT NULL,
    time    BIGINT NOT NULL,
    status  INTEGER NOT NULL,
    FOREIGN KEY (creator) REFERENCES users ON DELETE RESTRICT
) WITHOUT ROWID;

INSERT INTO conversations_tmp (
    id,
    name,
    ctype,
    creator,
    muted,
    time,
    status
) SELECT id, name, ctype, creator, muted, time, 0 FROM conversations;

DROP TABLE conversations;

ALTER TABLE conversations_tmp RENAME TO conversations;
