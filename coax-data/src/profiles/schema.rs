table! {
    profiles {
        id     -> Binary,
        time   -> BigInt,
        name   -> Text,
        handle -> Nullable<Text>,
        email  -> Nullable<Text>,
        phone  -> Nullable<Text>,
    }
}

pub const SCHEMA: &'static str = r"
    BEGIN;

    CREATE TABLE IF NOT EXISTS meta (
        version     INTEGER PRIMARY KEY,
        description TEXT,
        date        DATETIME
    ) WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS profiles (
        id      BLOB PRIMARY KEY,
        time    BIGINT NOT NULL,
        name    TEXT NOT NULL,
        handle  TEXT UNIQUE,
        email   TEXT UNIQUE,
        phone   TEXT UNIQUE
    ) WITHOUT ROWID;

    COMMIT;";

