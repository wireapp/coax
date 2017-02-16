use std::io::{self, Stderr, Read, Write};

/// Pass-through writes/reads and copy to stderr.
///
/// Generally only useful for debugging purposes.
pub struct Tee<T> {
    inner:  T,
    stderr: Stderr
}

impl<T> Tee<T> {
    pub fn new(t: T) -> Tee<T> {
        Tee {
            inner:  t,
            stderr: io::stderr()
        }
    }

    pub fn into(self) -> T {
        self.inner
    }
}

impl<T: Write> Write for Tee<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _ = self.stderr.write_all(buf);
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<T: Read> Read for Tee<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        let _ = self.stderr.write_all(&buf[0 .. n]);
        Ok(n)
    }
}
