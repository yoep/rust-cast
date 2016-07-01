#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate byteorder;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate protobuf;
extern crate serde;
extern crate serde_json;

pub mod cast;
pub mod errors;
mod utils;
mod message_manager;
pub mod channels;

use std::cell::RefCell;
use std::borrow::Cow;
use std::net::TcpStream;
use std::rc::Rc;

use openssl::ssl::{SslContext, SslStream, SslMethod};

use cast::cast_channel;

use channels::heartbeat::HeartbeatChannel;
use channels::connection::ConnectionChannel;
use channels::receiver::ReceiverChannel;
use channels::media::MediaChannel;

use errors::Error;

use message_manager::MessageManager;

const DEFAULT_SENDER_ID: &'static str = "sender-0";
const DEFAULT_RECEIVER_ID: &'static str = "receiver-0";

pub struct Chromecast {
    stream: Rc<RefCell<SslStream<TcpStream>>>,

    pub heartbeat: HeartbeatChannel<SslStream<TcpStream>>,
    pub connection: ConnectionChannel<SslStream<TcpStream>>,
    pub receiver: ReceiverChannel<SslStream<TcpStream>>,
}

impl Chromecast {
    pub fn connect(host: String, port: u16) -> Result<Chromecast, Error> {
        debug!("Establishing connection with Chromecast at {}:{}...", host, port);

        let ssl_context = try!(SslContext::new(SslMethod::Sslv23));
        let tcp_stream = try!(TcpStream::connect((host.as_ref(), port)));
        let ssl_stream = try!(SslStream::connect(&ssl_context, tcp_stream));

        debug!("Connection with {}:{} successfully established.", host, port);

        let ssl_stream_rc = Rc::new(RefCell::new(ssl_stream));

        let heartbeat = HeartbeatChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                              DEFAULT_RECEIVER_ID.to_owned(),
                                              ssl_stream_rc.clone());
        let connection = ConnectionChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                                ssl_stream_rc.clone());
        let receiver = ReceiverChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                            DEFAULT_RECEIVER_ID.to_owned(),
                                            ssl_stream_rc.clone());
        Ok(Chromecast {
            stream: ssl_stream_rc,
            heartbeat: heartbeat,
            connection: connection,
            receiver: receiver,
        })
    }

    pub fn receive(&self) -> Result<cast_channel::CastMessage, Error> {
        Ok(try!(MessageManager::receive(&mut *self.stream.borrow_mut())))
    }

    pub fn create_media_channel<'a, S>(&self, receiver: S, session_id: S)
        -> Result<MediaChannel<SslStream<TcpStream>>, Error> where S: Into<Cow<'a, str>> {
        Ok(MediaChannel::new(DEFAULT_SENDER_ID.to_owned(),
                             receiver.into().to_string(),
                             session_id.into().to_string(),
                             self.stream.clone()))
    }
}
