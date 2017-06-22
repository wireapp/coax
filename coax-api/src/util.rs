use std::borrow::Borrow;
use chrono::{DateTime, Utc};
use json::{Decoder, DecodeError, DecodeResult};
use json::ast::{Json, Ref};

pub fn parse_datetime<I>(d: &mut Decoder<I>) -> DecodeResult<DateTime<Utc>>
    where I: Iterator<Item=char>
{
    let s = d.string()?;
    match DateTime::parse_from_rfc3339(s.as_str()) {
        Ok(dt) => Ok(DateTime::from_utc(dt.naive_utc(), Utc)),
        Err(e) => Err(DecodeError::Other(Box::new(e)))
    }
}

pub fn datetime_from_json(j: &Json) -> DecodeResult<DateTime<Utc>> {
    match Ref::new(j).string().map(DateTime::parse_from_rfc3339) {
        Some(Ok(dt)) => Ok(DateTime::from_utc(dt.naive_utc(), Utc)),
        Some(Err(e)) => Err(DecodeError::Other(Box::new(e))),
        None         => Err(DecodeError::Message("expected date time"))
    }
}

pub fn map_json_slice<'a, R, T, F>(js: R, f: F) -> DecodeResult<Vec<T>>
    where F: Fn(&Json) -> DecodeResult<T>,
          R: Borrow<Ref<'a>>
{
    let v = js.borrow().slice().map(|sl| {
        let mut v = Vec::with_capacity(sl.len());
        for j in sl.iter() {
            v.push(f(j)?)
        }
        Ok(v)
    });
    Ok(from_some_ok!(v, DecodeError::Expected("json array")))
}

