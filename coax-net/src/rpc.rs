use std::error;
use std::io::{self, Read};

use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use json::{ToJson, Encoder, EncodeError};
use json::{FromJson, Decoder, DecodeError};
use json::decoder::{ReadIter, ReadError};
use url::Url;
use http::{Method, Stream, HttpStream};
use http::header::{self, Name, Value, Response, ContentEncoding};
use http::tls;
use http::transfer::{Chunk, ChunkReader, ChunkWriter, LenReader};
use slog::Logger;

// States
pub struct Init(());
pub struct Sent(());
pub struct Head(());

pub struct Rpc<'r, S> {
    stream:   HttpStream<S>,
    compress: bool,
    chunk:    Chunk,
    method:   Option<Method>,
    response: Response<'r>,
    log:      Logger
}

impl<'r, S: Stream> Rpc<'r, S> {
    pub fn new(g: &Logger, s: HttpStream<S>, compress: bool) -> (Rpc<'r, S>, Init) {
        let rpc = Rpc {
            stream:   s,
            compress: compress,
            chunk:    Chunk::new(),
            method:   None,
            response: Response::new(),
            log:      g.new(o!("context" => "Rpc"))
        };
        (rpc, Init(()))
    }

    pub fn response(&self) -> &Response {
        &self.response
    }

    pub fn stream(&self) -> &S {
        self.stream.stream()
    }

    pub fn set_stream(&mut self, s: S) -> Init {
        self.stream.set_stream(s);
        Init(())
    }

    pub fn set_compress(&mut self, compress: bool) {
        self.compress = compress
    }

    /// Send HTTP request without body.
    pub fn send(&mut self, _: Init, m: Method, u: &Url, h: &[(Name, Value)]) -> Result<Sent, Error> {
        debug!(self.log, "send"; "method" => m.as_str(), "url" => u.as_str());
        self.stream.request(m, u, h.iter().chain(DEFAULT_HEADERS.iter()))?;
        self.method = Some(m);
        Ok(Sent(()))
    }

    /// Send HTTP request header and JSON body.
    pub fn send_json<T: ToJson>(&mut self, _: Init, m: Method, u: &Url, h: &[(Name, Value)], body: &T) -> Result<Sent, Error> {
        debug!(self.log, "send json"; "method" => m.as_str(), "url" => u.as_str());
        let headers = h.iter().chain(DEFAULT_HEADERS.iter()).chain(JSON_HEADERS.iter());
        if self.compress {
            self.stream.request(m, u, headers.chain(GZIP_CONTENT.iter()))?;
        } else {
            self.stream.request(m, u, headers)?;
        }
        if self.compress {
            let mut w = ChunkWriter::new(&self.log, &mut self.stream, &mut self.chunk);
            let mut g = GzEncoder::new(w, Compression::Default);
            Encoder::new(&mut g).to_json(body)?;
            w = g.finish()?;
            w.finish()?;
        } else {
            let mut w = ChunkWriter::new(&self.log, &mut self.stream, &mut self.chunk);
            Encoder::new(&mut w).to_json(body)?;
            w.finish()?;
        }
        self.method = Some(m);
        Ok(Sent(()))
    }

    /// Receive HTTP response header.
    pub fn recv_header(&mut self, _: Sent) -> Result<Head, Error> {
        self.stream.response(&mut self.response)?;
        debug!(self.log, "header received"; "status" => self.response.status());
        Ok(Head(()))
    }

    /// Receive HTTP response body as JSON.
    pub fn recv_json<T: FromJson>(&mut self, h: Head) -> Result<(T, Init), Error> {
        trace!(self.log, "receive json");
        let mut r = self.reader(h)?;
        let x = read_json(ReadIter::new(&mut r))?;
        r.into().map(|i| (x, i)).map_err(From::from)
    }

    /// Consume HTTP response body.
    pub fn drain(&mut self, h: Head) -> Result<Init, Error> {
        trace!(self.log, "drain");
        self.reader(h)?.into().map_err(From::from)
    }

    /// HTTP response body reader.
    pub fn reader(&mut self, _: Head) -> Result<Reader<S>, Error> {
        if Some(Method::Head) == self.method {
            return Ok(Reader::new(Rdr::Empty(io::empty())))
        }
        let status = self.response.status();
        if status == 204 || status == 304 || status / 100 == 1 {
            return Ok(Reader::new(Rdr::Empty(io::empty())))
        }
        let encoding = self.response.content_encoding();
        // transfer-encoding: chunked
        if self.response.is_chunked() {
            let r = ChunkReader::new(&self.log, &mut self.stream);
            if encoding == Some(ContentEncoding::Gzip) {
                return Ok(Reader::new(Rdr::GzipCh(GzDecoder::new(r)?)))
            }
            return Ok(Reader::new(Rdr::Chunks(r)))
        }
        // content-length
        if let Some(Ok(n)) = self.response.content_length() {
            let r = LenReader::new(&self.log, &mut self.stream, n);
            if encoding == Some(ContentEncoding::Gzip) {
                return Ok(Reader::new(Rdr::GzipLn(GzDecoder::new(r)?)))
            }
            return Ok(Reader::new(Rdr::Length(r)))
        }
        // read until close
        if encoding == Some(ContentEncoding::Gzip) {
            return Ok(Reader::new(Rdr::GzipSt(GzDecoder::new(&mut self.stream)?)))
        }
        Ok(Reader::new(Rdr::Stream(&mut self.stream)))
    }
}

pub struct Reader<'a, S: Stream + 'a> {
    reader: Rdr<'a, S>
}

impl<'a, S: Stream + 'a> Reader<'a, S> {
    fn new(r: Rdr<'a, S>) -> Reader<'a, S> {
        Reader { reader: r }
    }

    pub fn into(mut self) -> io::Result<Init> {
        io::copy(&mut self, &mut io::sink())?;
        Ok(Init(()))
    }
}

enum Rdr<'a, S: Stream + 'a> {
    GzipCh(GzDecoder<ChunkReader<'a, S>>),
    GzipLn(GzDecoder<LenReader<'a, S>>),
    GzipSt(GzDecoder<&'a mut HttpStream<S>>),
    Chunks(ChunkReader<'a, S>),
    Length(LenReader<'a, S>),
    Stream(&'a mut HttpStream<S>),
    Empty(io::Empty)
}

impl<'a, S: Stream> Read for Reader<'a, S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = match self.reader {
            Rdr::GzipCh(ref mut r) => r.read(buf)?,
            Rdr::GzipLn(ref mut r) => r.read(buf)?,
            Rdr::GzipSt(ref mut r) => r.read(buf)?,
            Rdr::Chunks(ref mut r) => r.read(buf)?,
            Rdr::Length(ref mut r) => r.read(buf)?,
            Rdr::Stream(ref mut r) => r.read(buf)?,
            Rdr::Empty(ref mut r)  => r.read(buf)?
        };
        Ok(n)
    }
}

/// Read up to `max` bytes from `Read` type and drop excess bytes.
pub fn read_bytes<R: Read>(r: R, max: usize) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    let mut ctr = 0;
    for c in r.bytes() {
        if ctr < max {
            buf.push(c?);
            ctr += 1
        }
    }
    Ok(buf)
}

fn read_json<T: FromJson, R: Read>(mut i: ReadIter<R>) -> Result<T, Error> {
    let mut d = Decoder::default(&mut i);
    match d.from_json() {
        Ok(x)  => Ok(x),
        Err(e) => match d.into_iter().take_error() {
            Some(ReadError::InvalidUtf8) => Err(Error::Utf8),
            Some(ReadError::Io(e))       => Err(Error::Io(e)),
            None                         => Err(Error::JsonDecode(e))
        }
    }
}

const DEFAULT_HEADERS: &'static [(Name<'static>, Value<'static>)] = &[
    (header::ACCEPT_ENCODING, header::GZIP)
];

const JSON_HEADERS: &'static [(Name<'static>, Value<'static>)] = &[
    (header::ACCEPT, header::APPLICATION_JSON),
    (header::CONTENT_TYPE, header::APPLICATION_JSON),
    (header::TRANSFER_ENCODING, header::CHUNKED)
];

const GZIP_CONTENT: &'static [(Name<'static>, Value<'static>)] = &[
    (header::CONTENT_ENCODING, header::GZIP)
];

// Error type ///////////////////////////////////////////////////////////////

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(e: io::Error) {
            display("i/o error: {}", e)
            cause(e)
            from()
        }
        Tls(e: tls::Error) {
            display("tls error: {}", e)
            cause(e)
            from()
        }
        JsonEncode(e: EncodeError) {
            display("json encode error: {}", e)
            cause(e)
            from()
        }
        JsonDecode(e: DecodeError) {
            display("json decode error: {}", e)
            cause(e)
            from()
        }
        Other(e: Box<error::Error + Send + Sync>) {
            display("other error {}", e)
            cause(e.as_ref())
            from()
        }
        Response(m: String) {
            display("response error: {}", m)
        }
        Utf8 {
            display("utf-8 error")
        }
    }
}

