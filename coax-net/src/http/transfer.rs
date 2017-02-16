use std::cmp::min;
use std::io::{self, Read, Write, ErrorKind};

use httparse::{Status, parse_chunk_size};
use buf::Buf;
use slog::Logger;
use super::{Stream, HttpStream};

// Length Reader ////////////////////////////////////////////////////////////

pub struct LenReader<'a, S: 'a> {
    stream: &'a mut HttpStream<S>,
    len:    usize,
    ctr:    usize,
    log:    &'a Logger
}

impl<'a, S: Stream> LenReader<'a, S> {
    pub fn new(g: &'a Logger, h: &'a mut HttpStream<S>, n: usize) -> LenReader<'a, S> {
        trace!(g, "new length reader"; "len" => n);
        LenReader {
            stream: h,
            len:    n,
            ctr:    0,
            log:    g
        }
    }
}

impl<'a, S: Stream> Read for LenReader<'a, S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        trace!(self.log, "read bytes"; "buf-size" => buf.len());
        if self.ctr >= self.len {
            return Ok(0)
        }
        let k = self.stream.read(buf)?;
        self.ctr += k;
        Ok(k)
    }
}

// ChunkReader //////////////////////////////////////////////////////////////

pub struct ChunkReader<'a, S: 'a> {
    iter: super::Iter<'a, S>,
    len:  usize,
    rem:  usize,
    log:  &'a Logger,
    eof:  bool
}

impl<'a, S: Stream> ChunkReader<'a, S> {
    pub fn new(g: &'a Logger, h: &'a mut HttpStream<S>) -> ChunkReader<'a, S> {
        trace!(g, "new chunk reader");
        ChunkReader {
            iter: h.iter(),
            len:  0,
            rem:  0,
            log:  g,
            eof:  false
        }
    }

    fn next_chunk(&mut self) -> io::Result<usize> {
        trace!(self.log, "next_chunk");
        let mut b = [0; 18];
        let mut i = 0;
        for _ in 0 .. 16 {
            match self.next()? {
                Some(b';') => {
                    self.skip_while(|x| x != b'\r')?; // Ignore chunk extension.
                    b[i] = b'\r';
                    i += 1;
                    b[i] = self.demand()?;
                    if b[i] != b'\n' {
                        return Err(io::Error::new(ErrorKind::InvalidData, "invalid chunk header"))
                    }
                    i += 1;
                    break
                }
                Some(b'\r') => {
                    b[i] = b'\r';
                    i += 1;
                    b[i] = self.demand()?;
                    if b[i] != b'\n' {
                        return Err(io::Error::new(ErrorKind::InvalidData, "invalid chunk header"))
                    }
                    i += 1;
                    break
                }
                Some(x) => {
                    b[i] = x;
                    i += 1
                }
                None => break
            }
        }
        match parse_chunk_size(&b[0 .. i]) {
            Err(_) | Ok(Status::Partial) => {
                error!(self.log, "invalid chunk size");
                Err(io::Error::new(ErrorKind::InvalidData, "invalid chunk size"))
            }
            Ok(Status::Complete((_, c))) => {
                trace!(self.log, "chunk"; "size" => c);
                Ok(c as usize)
            }
        }
    }

    fn next(&mut self) -> io::Result<Option<u8>> {
        match self.iter.next() {
            None =>
                match self.iter.take_error() {
                    None    => Ok(None),
                    Some(e) => Err(e)
                },
            value => Ok(value)
        }
    }

    fn demand(&mut self) -> io::Result<u8> {
        match self.next()? {
            None    => Err(io::Error::new(ErrorKind::UnexpectedEof, "demand")),
            Some(x) => Ok(x)
        }
    }

    fn fill(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        for i in 0 .. buf.len() {
            match self.next()? {
                Some(x) => buf[i] = x,
                None    => return Ok(i)
            }
        }
        Ok(buf.len())
    }

    fn skip_while<F: Fn(u8) -> bool>(&mut self, f: F) -> io::Result<()> {
        while f(self.demand()?) { }
        Ok(())
    }

    fn matches(&mut self, pat: &[u8]) -> io::Result<bool> {
        for b in pat {
            if *b != self.demand()? {
                return Ok(false)
            }
        }
        Ok(true)
    }

    fn finish(&mut self) -> io::Result<()> {
        trace!(self.log, "finish chunks");
        loop {
            match self.demand()? {
                b'\r' =>
                    if b'\n' == self.demand()? {
                        return Ok(())
                    } else {
                        return Err(io::Error::new(ErrorKind::InvalidData, r"incomplete \r\n"))
                    },
                b';' => { // trailer
                    self.skip_while(|b| b != b'\r')?;
                    if b'\n' != self.demand()? {
                        return Err(io::Error::new(ErrorKind::InvalidData, r"incomplete \r\n"))
                    }
                }
                byte => {
                    error!(self.log, "unexpected chunk end"; "byte" => byte);
                    return Err(io::Error::new(ErrorKind::InvalidData, "unexpected chunk end"))
                }
            }
        }
    }
}

impl<'a, S: Stream> Read for ChunkReader<'a, S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        trace!(self.log, "read chunk bytes"; "buf" => buf.len());
        if self.eof {
            trace!(self.log, "chunk reader is at eof");
            return Ok(0)
        }
        if self.rem == 0 {
            self.len = self.next_chunk()?;
            self.rem = self.len;
            if self.rem == 0 { // eof
                trace!(self.log, "terminal chunk");
                self.finish()?;
                self.eof = true;
                return Ok(0)
            }
        }
        let n = min(buf.len(), self.rem);
        let k = self.fill(&mut buf[0 .. n])?;
        self.rem -= k;
        if self.rem == 0 { // end of current chunk
            if !self.matches(b"\r\n")? {
                return Err(io::Error::new(ErrorKind::InvalidData, r"no \r\n after chunk"))
            }
            self.len = self.next_chunk()?;
            self.rem = self.len;
            if self.rem == 0 {
                trace!(self.log, "no more chunks to come");
                self.finish()?;
                self.eof = true
            }
        }
        Ok(k)
    }
}

// Chunk ////////////////////////////////////////////////////////////////////

const CHUNK_HEADER_LEN: usize = 4;
const MAX_CHUNK_LEN: usize = 0x10000;

/// A single Chunk.
pub struct Chunk(Buf);

// We reserve the leading CHUNK_HEADER_LEN + 2 bytes for eventual
// storage of the chunk size header.
impl Chunk {
    pub fn new() -> Chunk {
        let mut c = Chunk(Buf::new(MAX_CHUNK_LEN));
        c.reset();
        c
    }

    pub fn reset(&mut self) {
        self.0.reset();
        self.0.extend(CHUNK_HEADER_LEN + 2);
        self.0.consume(CHUNK_HEADER_LEN + 2)
    }

    fn write_header(&mut self) -> io::Result<()> {
        let     e = self.0.end();
        let mut n = self.0.len();
        self.0.reset();
        let mut b = [b'0'; CHUNK_HEADER_LEN];
        let mut i = 0;
        while n > 0 {
            b[i] = to_hex(n % 16);
            n /= 16;
            i += 1;
        }
        b.reverse();
        self.0.write_all(&b)?;
        self.0.write_all(b"\r\n")?;
        assert!(self.0.set_end(e));
        Ok(())
    }
}

impl AsRef<[u8]> for Chunk {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

// ChunkWriter //////////////////////////////////////////////////////////////

/// Write `Transfer-Encoding: chunked` bytes.
pub struct ChunkWriter<'a, W> {
    writer: W,
    chunk:  &'a mut Chunk,
    log:    &'a Logger
}

impl<'a, W: Write> ChunkWriter<'a, W> {
    pub fn new(g: &'a Logger, w: W, c: &'a mut Chunk) -> ChunkWriter<'a, W> {
        trace!(g, "new chunk writer");
        c.reset();
        ChunkWriter {
            writer: w,
            chunk:  c,
            log:    g
        }
    }

    pub fn finish(&mut self) -> io::Result<()> {
        trace!(self.log, "finish chunks");
        self.flush()?;
        self.writer.write_all(b"0\r\n\r\n")?;
        self.writer.flush()
    }
}

impl<'a, W: Write> Write for ChunkWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        trace!(self.log, "write chunk bytes"; "data-size" => buf.len());
        if self.chunk.0.capacity() < buf.len() {
            self.flush()?;
        }
        self.chunk.0.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        trace!(self.log, "flush chunk writes");
        self.chunk.write_header()?;
        self.writer.write_all(self.chunk.as_ref())?;
        self.writer.write_all(b"\r\n")?;
        self.chunk.reset();
        self.writer.flush()
    }
}

fn to_hex(x: usize) -> u8 {
    match x {
        0  => b'0',
        1  => b'1',
        2  => b'2',
        3  => b'3',
        4  => b'4',
        5  => b'5',
        6  => b'6',
        7  => b'7',
        8  => b'8',
        9  => b'9',
        10 => b'a',
        11 => b'b',
        12 => b'c',
        13 => b'd',
        14 => b'e',
        15 => b'f',
        _  => panic!("to_hex: number out of range")
    }
}

// Tests ////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    extern crate quickcheck;

    use std::io::Write;
    use super::{CHUNK_HEADER_LEN, Chunk};

    #[test]
    fn test_chunk_header() {
        fn prop(n: usize) -> bool {
            let mut c = Chunk::new();
            for _ in 0 .. n {
                c.0.write(b"x").unwrap();
            }
            c.write_header().unwrap();
            &c.as_ref()[0 .. CHUNK_HEADER_LEN] == format!("{:04x}", n).as_bytes()
        }
        quickcheck::quickcheck(prop as fn(usize) -> bool)
    }
}
