-- Add encryption algorithm column to assets.
-- Make asset checksum optional.
-- Add mime-type to assets.

CREATE TABLE IF NOT EXISTS assets_tmp (
    id     TEXT PRIMARY KEY,
    atype  INTEGER NOT NULL,
    status INTEGER NOT NULL,
    token  TEXT,
    key    BLOB NOT NULL,
    cksum  BLOB,
    etype  INTEGER NOT NULL,
    mime   TEXT
) WITHOUT ROWID;

INSERT INTO assets_tmp (
    id,
    atype,
    status,
    token,
    key,
    cksum,
    etype
) SELECT id, atype, status, token, key, cksum, 0 FROM assets;

DROP TABLE assets;

ALTER TABLE assets_tmp RENAME TO assets;

PRAGMA foreign_key_check(assets);

