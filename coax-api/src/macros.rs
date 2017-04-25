#[macro_export]
macro_rules! json_str_type {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name<'a>(Cow<'a, str>);

        impl<'a> $name<'a> {
            pub fn new<S: Into<Cow<'a, str>>>(x: S) -> $name<'a> {
                $name(x.into())
            }

            pub fn as_str(&self) -> &str {
                self.0.borrow()
            }

            pub fn replicate(&self) -> $name {
                $name::new(self.0.borrow())
            }

            pub fn acquire<'b>(&'a self) -> $name<'b> {
                $name::new(Cow::Owned(String::from(self.as_str())))
            }
        }

        impl<'a> ToJson for $name<'a> {
            fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
                e.string(self.0.borrow())
            }
        }

        impl<'a> FromJson for $name<'a> {
            fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
                Ok($name(Cow::Owned(d.string()?)))
            }
        }

        impl<'a> ::std::fmt::Display for $name<'a> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }
    }
}


#[macro_export]
macro_rules! json_bytes_type {
    ($name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name<'a>(Cow<'a, [u8]>);

        impl<'a> $name<'a> {
            pub fn new<S: Into<Cow<'a, [u8]>>>(x: S) -> $name<'a> {
                $name(x.into())
            }

            pub fn as_bytes(&self) -> &[u8] {
                self.0.borrow()
            }
        }

        impl<'a> ToJson for $name<'a> {
            fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
                e.string(::base64::encode(&self.0))
            }
        }

        impl<'a> FromJson for $name<'a> {
            fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
                match ::base64::decode(d.string()?.as_str()) {
                    Ok(x)  => Ok($name(Cow::Owned(x))),
                    Err(e) => Err(DecodeError::Other(Box::new(e)))
                }
            }
        }
    }
}

macro_rules! from_some_ok {
    ($name:ident, $err:expr) => {
        match $name {
            Some(Ok(x))  => x,
            Some(Err(e)) => return Err(e),
            None         => return Err($err)
        }
    };
    ($name:ident) => {
        match $name {
            Some(Ok(x))  => Some(x),
            Some(Err(e)) => return Err(e),
            None         => None
        }
    }
}

macro_rules! from_some {
    ($name:ident, $err:expr) => {
        match $name {
            Some(x) => x,
            None    => return Err($err)
        }
    }
}

