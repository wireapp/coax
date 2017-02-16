use std::io;
use std::cmp::min;
use std::convert::{AsRef, AsMut};

pub struct Buf {
    buf: Box<[u8]>,
    cap: usize, // total capacity (never changes, == buf.len())
    end: usize, // end marker (pos <= end <= cap)
    pos: usize  // current position
}

impl Buf {
    pub fn new(n: usize) -> Buf {
        Buf {
            buf: vec![0; n].into_boxed_slice(),
            cap: n,
            end: 0,
            pos: 0
        }
    }

    /// Does this buffer contain any bytes to read?
    pub fn is_empty(&self) -> bool {
        self.pos == self.end
    }

    /// How many bytes can be read from this buffer?
    pub fn len(&self) -> usize {
        self.end - self.pos
    }

    /// The total length of this buffer.
    pub fn size(&self) -> usize {
        self.cap
    }

    /// How many bytes can be stored in this buffer?
    pub fn capacity(&self) -> usize {
        self.cap - self.end
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn end(&self) -> usize {
        self.end
    }

    /// Set end marker.
    ///
    /// Limited by `Buf::size()`. Returns true if we did
    /// not have to limit the argument, otherwise false.
    pub fn set_end(&mut self, n: usize) -> bool {
        let k = min(n, self.cap);
        self.end = k;
        k == n
    }

    /// Take one byte from the current position.
    pub fn take1(&mut self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            let x = self.buf[self.pos];
            self.pos += 1;
            Some(x)
        }
    }

    /// Reset end marker and current position to 0.
    ///
    /// After `reset`, the buffer is considered empty with
    /// full capacity.
    pub fn reset(&mut self) {
        self.end = 0;
        self.pos = 0
    }

    /// Move end marker by given amount.
    ///
    /// Limited by `Buf::capacity()`. Returns the actual number
    /// of positions the end marker could be moved.
    pub fn extend(&mut self, n: usize) -> usize {
        let k = min(n, self.capacity());
        self.end += min(n, k);
        k
    }

    /// Move forward by the given number of bytes.
    ///
    /// Limited by `Buf::len()`.
    pub fn consume(&mut self, n: usize) {
        self.pos += min(n, self.len())
    }
}

impl AsRef<[u8]> for Buf {
    fn as_ref(&self) -> &[u8] {
        &self.buf.as_ref()[self.pos .. self.end]
    }
}

impl AsMut<[u8]> for Buf {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.buf.as_mut()[self.end .. self.cap]
    }
}

impl io::Write for Buf {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.as_mut().write(buf)?;
        self.end += n;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Read for Buf {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.as_ref().read(buf)?;
        self.pos += n;
        Ok(n)
    }
}
