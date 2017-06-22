use chrono::{DateTime, Utc, NaiveDateTime};
use coax_api::types::Id;
use error::Error;

pub fn from_timestamp(t: i64) -> DateTime<Utc> {
    let ndt = NaiveDateTime::from_timestamp(t, 0);
    DateTime::from_utc(ndt, Utc)
}

pub fn as_id<T>(b: &[u8], m: &'static str) -> Result<Id<T>, Error> {
    Id::from_bytes(b).ok_or(Error::InvalidData(m))
}

