use std::io::{self, Read, Write};
use std::net::TcpStream;
use openssl::ssl::{self, SslConnectorBuilder, SslConnector};
use openssl::ssl::{SslStream, SslMethod};
use openssl::error::ErrorStack;
use super::AsTcp;

pub struct Tls(SslConnector);

// TODO: public key pinning
pub fn context() -> Result<Tls, Error> {
    let mut b = SslConnectorBuilder::new(SslMethod::tls())?;
    let     o = b.builder().options() | ssl::SSL_OP_NO_TLSV1;
    b.builder_mut().set_options(o);
    Ok(Tls(b.build()))
}

#[derive(Debug)]
pub struct TlsStream {
    tls: SslStream<TcpStream>
}

impl TlsStream {
    pub fn new(tls: &Tls, domain: &str, s: TcpStream) -> Result<TlsStream, Error> {
        let s = tls.0.connect(domain, s)?;
        Ok(TlsStream { tls: s })
    }
}

impl Read for TlsStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.tls.read(buf)
    }
}

impl Write for TlsStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.tls.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tls.flush()
    }
}

impl AsTcp for TlsStream {
    fn as_tcp(&self) -> &TcpStream {
        self.tls.get_ref()
    }
}

// Error type ///////////////////////////////////////////////////////////////

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Handshake(e: ssl::HandshakeError<TcpStream>) {
            display("tls handshake error: {}", e)
            cause(e)
            from()
        }
        Tls(e: ssl::Error) {
            display("tls error: {}", e)
            cause(e)
            from()
        }
        Openssl(e: ErrorStack) {
            display("openssl error stack: {}", e)
            cause(e)
            from()
        }
    }
}

