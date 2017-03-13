-- Remove asset column from messages.

CREATE TABLE IF NOT EXISTS messages_tmp (
    id       TEXT NOT NULL,
    conv     BLOB NOT NULL,
    time     BIGINT NOT NULL,
    from_usr BLOB NOT NULL,
    from_clt TEXT,
    mtype    INTEGER NOT NULL,
    status   INTEGER NOT NULL,
    text     TEXT,
    user_id  BLOB,

    PRIMARY KEY (conv, id),
    FOREIGN KEY (conv) REFERENCES conversations ON DELETE CASCADE
    FOREIGN KEY (from_usr) REFERENCES users ON DELETE RESTRICT
);

INSERT INTO messages_tmp (
    id,
    conv,
    time,
    from_usr,
    from_clt,
    mtype,
    status,
    text,
    user_id
) SELECT id, conv, time, from_usr, from_clt, mtype, status, text, user_id FROM messages;

DROP TABLE messages;

ALTER TABLE messages_tmp RENAME TO messages;

PRAGMA foreign_key_check(messages);
