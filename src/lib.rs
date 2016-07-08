#![doc(html_root_url = "https://azasypkin.github.io/rust-cast/")]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate byteorder;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate protobuf;
extern crate serde;
extern crate serde_json;

mod cast;
pub mod errors;
mod utils;
mod message_manager;
pub mod channels;

use std::borrow::Cow;
use std::cell::RefCell;
use std::net::TcpStream;
use std::rc::Rc;

use openssl::ssl::{SslContext, SslStream, SslMethod};

use channels::heartbeat::{HeartbeatChannel, HeartbeatResponse};
use channels::connection::{ConnectionChannel, ConnectionResponse};
use channels::receiver::{ReceiverChannel, ReceiverResponse};
use channels::media::{MediaChannel, MediaResponse};

use errors::Error;

use message_manager::MessageManager;

const DEFAULT_SENDER_ID: &'static str = "sender-0";
const DEFAULT_RECEIVER_ID: &'static str = "receiver-0";

/// Supported channel message types.
pub enum ChannelMessage<'a> {
    Connection(ConnectionResponse),
    Hearbeat(HeartbeatResponse),
    Media(MediaResponse<'a>),
    Receiver(ReceiverResponse),
    Raw(cast::cast_channel::CastMessage),
}

/// Structure that manages connection to a cast device.
pub struct CastDevice<'a> {
    stream: Rc<RefCell<SslStream<TcpStream>>>,

    /// Channel that manages connection responses/requests.
    pub connection: ConnectionChannel<'a, SslStream<TcpStream>>,

    /// Channel that allows connection to stay alive (via ping-pong requests/responses).
    pub heartbeat: HeartbeatChannel<'a, SslStream<TcpStream>>,

    /// Channel that manages various media stuff.
    pub media: MediaChannel<'a, SslStream<TcpStream>>,

    /// Channel that manages receiving platform (eg. Chromecast).
    pub receiver: ReceiverChannel<'a, SslStream<TcpStream>>,
}

impl<'a> CastDevice<'a> {
    /// Connects to the cast device using host name and port.
    ///
    /// # Examples
    ///
    /// ```
    /// let device = try!(CastDevice::connect(args.flag_address.unwrap(), args.flag_port));
    /// ```
    ///
    /// # Arguments
    ///
    /// * `host` - Cast device host name.
    /// * `port` - Cast device port number.
    ///
    /// # Errors
    ///
    /// This method may fail if connection to Cast device can't be established for some reason
    /// (eg. wrong host name or port).
    ///
    /// # Return value
    ///
    /// Instance of `CastDevice` that allows you to manage connection.
    pub fn connect<S>(host: S, port: u16)
        -> Result<CastDevice<'a>, Error> where S: Into<Cow<'a, str>> {
        let host = host.into();

        debug!("Establishing connection with cast device at {}:{}...", host, port);

        let ssl_context = try!(SslContext::new(SslMethod::Sslv23));
        let tcp_stream = try!(TcpStream::connect((host.as_ref(), port)));
        let ssl_stream = try!(SslStream::connect(&ssl_context, tcp_stream));

        debug!("Connection with {}:{} successfully established.", host, port);

        let ssl_stream_rc = Rc::new(RefCell::new(ssl_stream));

        let heartbeat = HeartbeatChannel::new(DEFAULT_SENDER_ID, DEFAULT_RECEIVER_ID,
                                              ssl_stream_rc.clone());
        let connection = ConnectionChannel::new(DEFAULT_SENDER_ID, ssl_stream_rc.clone());
        let receiver = ReceiverChannel::new(DEFAULT_SENDER_ID, DEFAULT_RECEIVER_ID,
                                            ssl_stream_rc.clone());
        let media = MediaChannel::new(DEFAULT_SENDER_ID, ssl_stream_rc.clone());

        Ok(CastDevice {
            stream: ssl_stream_rc,
            heartbeat: heartbeat,
            connection: connection,
            receiver: receiver,
            media: media,
        })
    }

    /// Waits for any message returned by cast device (eg. Chromecast) and returns its parsed
    /// version.
    ///
    /// # Examples
    ///
    /// ```
    /// match cast_device.receive() {
    ///     Ok(ChannelMessage::Connection(res)) => debug!("Connection message: {:?}", res),
    ///     Ok(ChannelMessage::Heartbeat(_)) => cast_device.heartbeat.pong(),
    ///     .......
    ///     Err(err) => error!("Error occurred while receiving message {}", err)
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Usually fails if message returned by device can't be parsed.
    ///
    /// # Returned values
    ///
    /// Parsed channel message.
    pub fn receive(&self) -> Result<ChannelMessage, Error> {
        let cast_message = try!(MessageManager::receive(&mut *self.stream.borrow_mut()));

        if self.connection.can_handle(&cast_message) {
            return Ok(ChannelMessage::Connection(try!(self.connection.parse(&cast_message))));
        }

        if self.heartbeat.can_handle(&cast_message) {
            return Ok(ChannelMessage::Hearbeat(try!(self.heartbeat.parse(&cast_message))));
        }

        if self.media.can_handle(&cast_message) {
            return Ok(ChannelMessage::Media(try!(self.media.parse(&cast_message))));
        }

        if self.receiver.can_handle(&cast_message) {
            return Ok(ChannelMessage::Receiver(try!(self.receiver.parse(&cast_message))));
        }

        Ok(ChannelMessage::Raw(cast_message))
    }
}
