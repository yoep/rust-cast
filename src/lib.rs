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
        let message = MessageManager::receive(&mut *self.stream.borrow_mut());

        debug!("Message received: {:?}", message);

        Ok(message)
    }

    pub fn create_media_channel(&self, receiver: String, session_id: String)
        -> Result<MediaChannel<SslStream<TcpStream>>, Error> {
        Ok(MediaChannel::new(DEFAULT_SENDER_ID.to_owned(),
                             receiver,
                             session_id,
                             self.stream.clone()))
    }
}
