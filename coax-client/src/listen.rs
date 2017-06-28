use std::io::Cursor;

use coax_api::events::Notification;
use coax_api::token::AccessToken;
use futures::{Async, AsyncSink, Future, Poll, Sink, Stream};
use json::decoder::{Decoder, ReadIter};
use error::{Error, Void};
use native_tls::TlsConnector;
use slog::Logger;
use tokio_core::reactor::Handle;
use url::Url;
use websocket::{ClientBuilder, OwnedMessage};
use websocket::async::TcpStream;
use websocket::client::async::{Client, TlsStream};
use websocket::header::{Headers, Authorization, Bearer};

// Listener /////////////////////////////////////////////////////////////////

pub struct Listener {
    logger:  Logger,
    client:  Client<TlsStream<TcpStream>>,
    outbox:  Option<OwnedMessage>,
    counter: u64
}

impl Listener {
    pub fn open(g: &Logger, url: Url, tls: TlsConnector, tkn: &AccessToken, hdl: &Handle) -> Result<Listener, Error<Void>> {
        debug!(g, "open websocket"; "url" => %url);

        let mut h = Headers::new();
        h.set(Authorization(Bearer { token: tkn.bearer.to_owned() }));

        let client = ClientBuilder::from_url(&url)
            .custom_headers(&h)
            .async_connect_secure(Some(tls), hdl)
            .map(|x| x.0)
            .wait()?;

        Ok(Listener {
            logger:  g.new(o!("context" => "Listener")),
            client:  client,
            outbox:  None,
            counter: 0
        })
    }

    pub fn counter(&self) -> u64 {
        self.counter
    }
}

impl Stream for Listener {
    type Item  = Notification<'static>;
    type Error = Error<Void>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            if let Some(item) = self.outbox.take() {
                match self.client.start_send(item) {
                    Ok(AsyncSink::NotReady(item)) => {
                        self.outbox = Some(item);
                        return Ok(Async::NotReady)
                    }
                    Ok(AsyncSink::Ready) => {}
                    Err(e)               => return Err(e.into())
                }
            }

            try_ready!(self.client.poll_complete());

            match try_ready!(self.client.poll()) {
                Some(OwnedMessage::Text(s)) => {
                    debug!(self.logger, "text received");
                    self.counter.wrapping_add(1);
                    return match Decoder::default(s.chars()).from_json() {
                        Ok(n)  => Ok(Async::Ready(Some(n))),
                        Err(e) => Err(e.into())
                    }
                }
                Some(OwnedMessage::Binary(b)) => {
                    debug!(self.logger, "bytes received");
                    self.counter.wrapping_add(1);
                    return match Decoder::default(ReadIter::new(Cursor::new(b))).from_json() {
                        Ok(n)  => Ok(Async::Ready(Some(n))),
                        Err(e) => Err(e.into())
                    }
                }
                Some(OwnedMessage::Ping(p)) => {
                    debug!(self.logger, "ping received");
                    self.counter.wrapping_add(1);
                    self.outbox = Some(OwnedMessage::Pong(p));
                    continue
                }
                Some(OwnedMessage::Pong(_)) => {
                    debug!(self.logger, "pong received");
                    self.counter.wrapping_add(1);
                }
                Some(OwnedMessage::Close(d)) => {
                    info!(self.logger, "close received");
                    self.outbox = Some(OwnedMessage::Close(d));
                    continue
                }
                None => return Ok(Async::NotReady)
            }
        }
    }
}

