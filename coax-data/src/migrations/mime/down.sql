-- Remove encryption algorithm column from assets.
-- Remove mime-type from assets.
-- Make asset checksum non-optional.

CREATE TABLE IF NOT EXISTS assets_tmp (
    id     TEXT PRIMARY KEY,
    atype  INTEGER NOT NULL,
    status INTEGER NOT NULL,
    token  TEXT,
    key    BLOB NOT NULL,
    cksum  BLOB NOT NULL
) WITHOUT ROWID;

INSERT INTO assets_tmp (
    id,
    atype,
    status,
    token,
    key,
    cksum
) SELECT id, atype, status, token, key, ifnull(cksum, x'') FROM assets;

DROP TABLE assets;

ALTER TABLE assets_tmp RENAME TO assets;

PRAGMA foreign_key_check(assets);
