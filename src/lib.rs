#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate openssl;
extern crate protobuf;
extern crate serde;
extern crate serde_json;

pub mod utils;
pub mod channels;
pub mod message_manager;
pub mod cast;

use std::cell::RefCell;
use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::rc::Rc;

use openssl::ssl::{SslContext, SslStream, SslMethod};

use cast::cast_channel;

use channels::heartbeat::HeartbeatChannel;
use channels::connection::ConnectionChannel;
use channels::receiver::ReceiverChannel;
use channels::media::MediaChannel;

use message_manager::MessageManager;

const DEFAULT_SENDER_ID: &'static str = "sender-0";
const DEFAULT_RECEIVER_ID: &'static str = "receiver-0";

pub struct Chromecast {
    host: String,
    port: u16,
    pub stream: Option<Rc<RefCell<SslStream<TcpStream>>>>,

    pub heartbeat: Option<HeartbeatChannel<SslStream<TcpStream>>>,
    pub connection: Option<ConnectionChannel<SslStream<TcpStream>>>,
    pub receiver: Option<ReceiverChannel<SslStream<TcpStream>>>,
}

impl Chromecast {
    pub fn new(host: String, port: u16) -> Chromecast {
        Chromecast {
            host: host,
            port: port,
            stream: None,
            heartbeat: None,
            connection: None,
            receiver: None,
        }
    }

    pub fn connect(&mut self) -> io::Result<()> {
        let address: Vec<_> = (&self.host as &str, self.port)
                                  .to_socket_addrs()
                                  .unwrap()
                                  .collect();

        let ssl_context = match SslContext::new(SslMethod::Sslv23) {
            Ok(context) => context,
            Err(err) => {
                let error_string = format!("Failed to create SSL Context: {}", err);
                return Err(io::Error::new(io::ErrorKind::Other, error_string));
            }
        };

        let tcp_stream = try!(TcpStream::connect(address[0]));

        let ssl_stream = match SslStream::connect(&ssl_context, tcp_stream) {
            Ok(stream) => stream,
            Err(err) => {
                let error_string = format!("Failed to create SSL Stream: {}", err);
                return Err(io::Error::new(io::ErrorKind::Other, error_string));
            }
        };

        let ssl_stream_rc = Rc::new(RefCell::new(ssl_stream));

        self.heartbeat = Some(HeartbeatChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                                    DEFAULT_RECEIVER_ID.to_owned(),
                                                    ssl_stream_rc.clone()));
        self.connection = Some(ConnectionChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                                      ssl_stream_rc.clone()));
        self.receiver = Some(ReceiverChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                                  DEFAULT_RECEIVER_ID.to_owned(),
                                                  ssl_stream_rc.clone()));

        self.stream = Some(ssl_stream_rc);

        Ok(())
    }

    pub fn receive(&mut self) -> cast_channel::CastMessage {
        if let Some(stream_rc) = self.stream.as_ref() {
            MessageManager::receive(&mut *stream_rc.borrow_mut())
        } else {
            panic!("Chromecast is not connected!");
        }
    }

    pub fn create_heartbeat_channel(&self) -> HeartbeatChannel<SslStream<TcpStream>> {
        if let Some(stream_rc) = self.stream.as_ref() {
            HeartbeatChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                  DEFAULT_RECEIVER_ID.to_owned(),
                                  stream_rc.clone())
        } else {
            panic!("Chromecast is not connected!");
        }
    }

    pub fn create_connection_channel(&self) -> ConnectionChannel<SslStream<TcpStream>> {
        if let Some(stream_rc) = self.stream.as_ref() {
            ConnectionChannel::new(DEFAULT_SENDER_ID.to_owned(), stream_rc.clone())
        } else {
            panic!("Chromecast is not connected!");
        }
    }

    pub fn create_receiver_channel(&self) -> ReceiverChannel<SslStream<TcpStream>> {
        if let Some(stream_rc) = self.stream.as_ref() {
            ReceiverChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                 DEFAULT_RECEIVER_ID.to_owned(),
                                 stream_rc.clone())
        } else {
            panic!("Chromecast is not connected!");
        }
    }

    pub fn create_media_channel(&self,
                                receiver: String,
                                session_id: String)
                                -> MediaChannel<SslStream<TcpStream>> {
        if let Some(stream_rc) = self.stream.as_ref() {
            MediaChannel::new(DEFAULT_SENDER_ID.to_owned(),
                              receiver,
                              session_id,
                              stream_rc.clone())
        } else {
            panic!("Chromecast is not connected!");
        }
    }
}
