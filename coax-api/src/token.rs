use std::borrow::Cow;
use std::time::{Duration, Instant};

use json::{FromJson, Decoder, DecodeResult, Utf8Buffer};

// Access token /////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct AccessToken<'a>{
    pub token:     Cow<'a, str>,
    pub created:   Instant,
    pub valid_for: Duration,
    pub bearer:    String
}

impl<'a> AccessToken<'a> {
    pub fn new<T>(t: T, dur: Duration) -> AccessToken<'a>
        where T: Into<Cow<'a, str>>
    {
        let now = Instant::now();
        let tkn = t.into();
        let ber = format!("Bearer {}", tkn);
        AccessToken {
            token:     tkn,
            created:   now,
            valid_for: dur,
            bearer:    ber
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created.elapsed() > self.valid_for
    }

    pub fn valid_for(&self, d: Duration) -> bool {
        self.valid_for - self.created.elapsed() > d
    }
}

impl<'a> FromJson for AccessToken<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        let tkn = extract! {
            let decoder = d;
            let buffer  = &mut u;
            token:    req. String   = "access_token" => d.string(),
            duration: req. Duration = "expires_in"   => d.u64().map(Duration::from_secs)
        }?;
        Ok(AccessToken::new(tkn.token, tkn.duration))
    }
}

#[derive(Debug)]
pub struct Credentials<'a, C> {
    pub token:  AccessToken<'a>,
    pub cookie: C
}

impl<'a, C> Credentials<'a, C> {
    pub fn new(t: AccessToken<'a>, c: C) -> Credentials<'a, C> {
        Credentials {
            token:  t,
            cookie: c
        }
    }
}

pub mod renew {

    use types::ApiError;

    quick_error! {
        #[derive(Debug)]
        /// Renew error
        pub enum Error {
            Invalid {
                display("invalid Credentials")
            }
            Other(e: ApiError<'static>) {
                display("api error: {}", e)
                cause(e)
            }
        }
    }

    impl From<ApiError<'static>> for Error {
        fn from(e: ApiError<'static>) -> Error {
            match e.code {
                403 => Error::Invalid,
                _   => Error::Other(e)
            }
        }
    }
}
