use diesel::connection::SimpleConnection;
use diesel::migrations::{Migration, RunMigrationsError};

macro_rules! migration {
    ($version:expr, $name:ident, $up:expr, $down:expr) => {
        pub struct $name();

        impl Migration for $name {
            fn version(&self) -> &str { $version }

            fn run(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
                conn.batch_execute(include_str!($up)).map_err(From::from)
            }

            fn revert(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
                conn.batch_execute(include_str!($down)).map_err(From::from)
            }
        }
    }
}

migration!("2017-03-08+001", ConversationStatus, "convstatus/up.sql", "convstatus/down.sql");
migration!("2017-03-09+001", MessageUserId, "msguserid/up.sql", "msguserid/down.sql");
migration!("2017-03-14+001", MessageAssetKey, "msgasset/up.sql", "msgasset/down.sql");

pub fn all() -> Vec<Box<Migration>> {
    vec![
        Box::new(ConversationStatus()),
        Box::new(MessageUserId()),
        Box::new(MessageAssetKey())
    ]
}
