use chrono::Utc;
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::sqlite::query_builder::functions::insert_or_replace;
use error::Error;
use slog::Logger;
use super::model::User;

mod model;
mod schema;

pub use self::model::Profile;

pub struct ProfileDb {
    logger: Logger,
    conn:   SqliteConnection
}

impl ProfileDb {
    pub fn open(g: &Logger, path: &str) -> Result<ProfileDb, Error> {
        let c = SqliteConnection::establish(path)?;
        let db = ProfileDb {
            logger: g.new(o!("context" => "ProfileDb")),
            conn:   c
        };
        db.conn.batch_execute(r#"PRAGMA foreign_keys = ON; PRAGMA journal_mode=WAL;"#)?;
        Ok(db)
    }

    pub fn setup_schema(&self) -> Result<(), Error> {
        self.conn.batch_execute(schema::SCHEMA)?;
        Ok(())
    }

    pub fn insert(&self, u: &User) -> Result<(), Error> {
        debug!(self.logger, "inserting profile"; "id" => %u.id);
        let p = model::NewProfile {
            id:     u.id.as_slice(),
            time:   Utc::now().timestamp(),
            name:   u.name.as_str(),
            handle: None, // TODO
            email:  u.email.as_ref().map(|e| e.as_str()),
            phone:  u.phone.as_ref().map(|p| p.as_str())
        };
        insert_or_replace(&p).into(schema::profiles::table).execute(&self.conn)?;
        Ok(())
    }

    pub fn select<'a>(&self) -> Result<Vec<Profile<'a>>, Error> {
        debug!(self.logger, "selecting profiles");
        let pp = schema::profiles::table.load::<model::RawProfile>(&self.conn)?;
        let mut v = Vec::with_capacity(pp.len());
        for p in pp {
            v.push(p.to_profile()?)
        }
        Ok(v)
    }
}

