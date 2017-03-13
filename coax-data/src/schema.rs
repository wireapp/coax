table! {
    users {
        id     -> Binary,
        name   -> Text,
        state  -> SmallInt,
        handle -> Nullable<Text>,
        email  -> Nullable<Text>,
        phone  -> Nullable<Text>,
        icon   -> Nullable<Text>,
    }
}

table! {
    clients (user, id) {
        id       -> Text,
        user     -> Binary,
        class    -> Nullable<SmallInt>,
        verified -> Bool,
        time     -> Nullable<BigInt>,
        ctype    -> Nullable<SmallInt>,
        label    -> Nullable<Text>,
        model    -> Nullable<Text>,
    }
}

table! {
    connections {
        id      -> Binary,
        conv    -> Binary,
        status  -> SmallInt,
        message -> Nullable<Text>,
    }
}

table! {
    conversations {
        id      -> Binary,
        name    -> Nullable<Text>,
        ctype   -> SmallInt,
        creator -> Binary,
        muted   -> Bool,
        time    -> BigInt,
        status  -> SmallInt,
    }
}

table! {
    members (conv, id) {
        id   -> Binary,
        conv -> Binary,
    }
}

table! {
    messages (conv, id) {
        id        -> Text,
        conv      -> Binary,
        time      -> BigInt,
        from_usr  -> Binary,
        from_clt  -> Nullable<Text>,
        mtype     -> SmallInt,
        status    -> SmallInt,
        text      -> Nullable<Text>,
        user_id   -> Nullable<Binary>,
        asset     -> Nullable<Text>,
    }
}

table! {
    assets {
        id     -> Text,
        atype  -> SmallInt,
        status -> SmallInt,
        token  -> Nullable<Text>,
        key    -> Binary,
        cksum  -> Binary,
    }
}

table! {
    variables (name) {
        name  -> Text,
        value -> Binary,
    }
}

table! {
    inbox {
        id -> Binary,
    }
}

table! {
    outbox (conv, id) {
        id   -> Binary,
        conv -> Binary,
        kind -> SmallInt,
        data -> Binary,
        mesg -> Nullable<Binary>,
    }
}

pub const SCHEMA: &'static str = r"
    BEGIN;

    CREATE TABLE IF NOT EXISTS __diesel_schema_migrations (
        version VARCHAR(50) PRIMARY KEY NOT NULL,
        run_on  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    ) WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS inbox (
        id BLOB PRIMARY KEY
    ) WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS outbox (
        id    BLOB NOT NULL,
        conv  BLOB NOT NULL,
        kind  INTEGER NOT NULL,
        data  BLOB NOT NULL,
        mesg  BLOB,

        PRIMARY KEY (conv, id)
    );

    CREATE TABLE IF NOT EXISTS users (
        id      BLOB PRIMARY KEY,
        name    TEXT NOT NULL,
        state   INTEGER NOT NULL,
        handle  TEXT UNIQUE,
        email   TEXT UNIQUE,
        phone   TEXT UNIQUE,
        icon    TEXT
    ) WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS clients (
        id       TEXT,
        user     BLOB,
        class    INTEGER,
        verified BOOLEAN NOT NULL,
        time     BIGINT,
        ctype    INTEGER,
        label    TEXT,
        model    TEXT,

        PRIMARY KEY (user, id),
        FOREIGN KEY (user) REFERENCES users ON DELETE CASCADE
    ) WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS connections (
        id      BLOB PRIMARY KEY,
        conv    BLOB NOt NULL,
        status  INTEGER NOT NULL,
        message TEXT,

        FOREIGN KEY (id) REFERENCES users ON DELETE CASCADE
    ) WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS conversations (
        id      BLOB PRIMARY KEY,
        name    TEXT,
        ctype   INTEGER NOT NULL,
        creator BLOB NOT NULL,
        muted   BOOLEAN NOT NULL,
        time    BIGINT NOT NULL,
        status  INTEGER NOT NULL,

        FOREIGN KEY (creator) REFERENCES users ON DELETE RESTRICT
    ) WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS members (
        id   BLOB,
        conv BLOB,

        PRIMARY KEY (conv, id),
        FOREIGN KEY (id) REFERENCES users ON DELETE RESTRICT,
        FOREIGN KEY (conv) REFERENCES conversations ON DELETE CASCADE
    ) WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS messages (
        id       TEXT NOT NULL,
        conv     BLOB NOT NULL,
        time     BIGINT NOT NULL,
        from_usr BLOB NOT NULL,
        from_clt TEXT,
        mtype    INTEGER NOT NULL,
        status   INTEGER NOT NULL,
        text     TEXT,

        PRIMARY KEY (conv, id),
        FOREIGN KEY (conv) REFERENCES conversations ON DELETE CASCADE
        FOREIGN KEY (from_usr) REFERENCES users ON DELETE RESTRICT
    );

    CREATE INDEX IF NOT EXISTS message_id_index ON messages (id);

    CREATE TABLE IF NOT EXISTS assets (
        id     TEXT PRIMARY KEY,
        atype  INTEGER NOT NULL,
        status INTEGER NOT NULL,
        token  TEXT,
        key    BLOB NOT NULL,
        cksum  BLOB NOT NULL
    ) WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS variables (
        name  TEXT PRIMARY KEY,
        value BLOB NOT NULL
    ) WITHOUT ROWID;

    COMMIT;";
