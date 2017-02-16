use chrono::{DateTime, UTC};
use coax_api::types::{UserId, Name, Email, Phone};
use error::Error;
use super::schema::profiles;
use util::{from_timestamp, as_id};

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "profiles"]
pub struct RawProfile {
    pub id:     Vec<u8>,
    pub time:   i64,
    pub name:   String,
    pub handle: Option<String>,
    pub email:  Option<String>,
    pub phone:  Option<String>
}

impl RawProfile {
    pub fn to_profile<'a>(self) -> Result<Profile<'a>, Error> {
        Ok(Profile {
            id:     as_id(&self.id, "user id")?,
            time:   from_timestamp(self.time),
            name:   Name::new(self.name),
            handle: self.handle.map(Name::new),
            email:  self.email.map(Email::new),
            phone:  self.phone.map(Phone::new),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Profile<'a> {
    pub id:     UserId,
    pub time:   DateTime<UTC>,
    pub name:   Name<'a>,
    pub handle: Option<Name<'a>>,
    pub email:  Option<Email<'a>>,
    pub phone:  Option<Phone<'a>>
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "profiles"]
pub struct NewProfile<'a> {
    pub id:       &'a [u8],
    pub time:     i64,
    pub name:     &'a str,
    pub handle:   Option<&'a str>,
    pub email:    Option<&'a str>,
    pub phone:    Option<&'a str>
}

