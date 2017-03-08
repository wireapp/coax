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

migration!("1", ConversationStatus, "convstatus/up.sql", "convstatus/down.sql");

pub fn all() -> Vec<Box<Migration>> {
    vec![Box::new(ConversationStatus())]
}
