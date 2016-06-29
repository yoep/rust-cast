#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

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

pub struct ChromecastAppsNames<'a> {
    pub backdrop: &'a str,
    pub default_media_receiver: &'a str,
    pub youtube: &'a str,
    #[allow(dead_code)]
    private: ()
}

pub const CHROMECAST_APPS: ChromecastAppsNames<'static> = ChromecastAppsNames {
    backdrop: "E8C28D3C",
    default_media_receiver: "CC1AD845",
    youtube: "YouTube",
    private: (),
};

pub enum ChromecastApps {
    DefaultMediaReceiver,
    YouTube,
}

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

    pub fn connect(&mut self) -> Result<(), Error> {
        let ssl_context = try!(SslContext::new(SslMethod::Sslv23));
        let tcp_stream = try!(TcpStream::connect((self.host.as_ref(), self.port)));
        let ssl_stream = try!(SslStream::connect(&ssl_context, tcp_stream));

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

    pub fn receive(&mut self) -> Result<cast_channel::CastMessage, Error> {
        if self.stream.is_none() {
            return Err(Error::Internal("Chromecast is not connected!".to_owned()));
        }

        let stream = self.stream.as_ref().unwrap();
        let message = MessageManager::receive(&mut *stream.borrow_mut());

        debug!("Message received: {:?}", message);

        Ok(message)
    }

    pub fn create_heartbeat_channel(&self)
        -> Result<HeartbeatChannel<SslStream<TcpStream>>, Error> {
        if self.stream.is_none() {
            return Err(Error::Internal("Chromecast is not connected!".to_owned()));
        }

        let stream = self.stream.as_ref().unwrap();

        Ok(HeartbeatChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                 DEFAULT_RECEIVER_ID.to_owned(),
                                 stream.clone()))
    }

    pub fn create_connection_channel(&self)
        -> Result<ConnectionChannel<SslStream<TcpStream>>, Error> {
        if self.stream.is_none() {
            return Err(Error::Internal("Chromecast is not connected!".to_owned()));
        }

        let stream = self.stream.as_ref().unwrap();
        Ok(ConnectionChannel::new(DEFAULT_SENDER_ID.to_owned(), stream.clone()))
    }

    pub fn create_receiver_channel(&self) -> Result<ReceiverChannel<SslStream<TcpStream>>, Error> {
        if self.stream.is_none() {
            return Err(Error::Internal("Chromecast is not connected!".to_owned()));
        }

        let stream = self.stream.as_ref().unwrap();
        Ok(ReceiverChannel::new(DEFAULT_SENDER_ID.to_owned(),
                                DEFAULT_RECEIVER_ID.to_owned(),
                                stream.clone()))
    }

    pub fn create_media_channel(&self, receiver: String, session_id: String)
        -> Result<MediaChannel<SslStream<TcpStream>>, Error> {
        if self.stream.is_none() {
            return Err(Error::Internal("Chromecast is not connected!".to_owned()));
        }

        let stream = self.stream.as_ref().unwrap();
        Ok(MediaChannel::new(DEFAULT_SENDER_ID.to_owned(),
                             receiver,
                             session_id,
                             stream.clone()))
    }
}
