#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate openssl;
extern crate protobuf;
extern crate serde;
extern crate serde_json;

mod cast;
mod utils;

use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs, SocketAddr};
use std::io;

use openssl::ssl::{SslContext, SslStream, SslMethod};

use cast::requests;
use cast::cast_channel::*;

use protobuf::*;

const DEFAULT_SOURCE_ID: &'static str = "chromecast-link-0";
const PLATFORM_DESTINATION_ID: &'static str = "receiver-0";

const NS_CONNECTION: &'static str = "urn:x-cast:com.google.cast.tp.connection";
const NS_HEARTBEAT: &'static str = "urn:x-cast:com.google.cast.tp.heartbeat";
const NS_RECEIVER: &'static str = "urn:x-cast:com.google.cast.receiver";
const NS_MEDIA: &'static str = "urn:x-cast:com.google.cast.media";

const MESSAGE_TYPE_PING: &'static str = "PING";
const MESSAGE_TYPE_RECEIVER_STATUS: &'static str = "RECEIVER_STATUS";
const MESSAGE_TYPE_PONG: &'static str = "PONG";
const MESSAGE_TYPE_CONNECT: &'static str = "CONNECT";
const MESSAGE_TYPE_CLOSE: &'static str = "CLOSE";
const MESSAGE_TYPE_GET_STATUS: &'static str = "GET_STATUS";
const MESSAGE_TYPE_LAUNCH: &'static str = "LAUNCH";
const MESSAGE_TYPE_LAUNCH_ERROR: &'static str = "LAUNCH_ERROR";
const MESSAGE_TYPE_LOAD: &'static str = "LOAD";

const STREAM_TYPE_UNKNOWN: &'static str = "UNKNOWN";
const STREAM_TYPE_BUFFERED: &'static str = "BUFFERED";
const STREAM_TYPE_LIVE: &'static str = "LIFE";

const APP_BACKDROP: &'static str = "E8C28D3C";
const APP_YOUTUBE: &'static str = "YouTube";
const APP_MEDIA_RECEIVER: &'static str = "CC1AD845";

fn create_message(namespace: &str,
                  source: &str,
                  destination: &str,
                  receiver_respons: String)
                  -> CastMessage {
    let mut message = CastMessage::new();

    message.set_protocol_version(CastMessage_ProtocolVersion::CASTV2_1_0);

    message.set_namespace(namespace.to_owned());
    message.set_source_id(source.to_owned());
    message.set_destination_id(destination.to_owned());

    message.set_payload_type(CastMessage_PayloadType::STRING);
    message.set_payload_utf8(receiver_respons);

    message
}

fn get_connect_message(destination: &str) -> CastMessage {
    let request = requests::GenericRequest { typ: MESSAGE_TYPE_CONNECT.to_owned() };

    create_message(NS_CONNECTION,
                   DEFAULT_SOURCE_ID,
                   destination,
                   serde_json::to_string(&request).unwrap())
}

fn get_ping_message() -> CastMessage {
    let request = requests::GenericRequest { typ: MESSAGE_TYPE_PING.to_owned() };

    create_message(NS_HEARTBEAT,
                   DEFAULT_SOURCE_ID,
                   PLATFORM_DESTINATION_ID,
                   serde_json::to_string(&request).unwrap())
}

fn get_app_launch_message(app_id: &str) -> CastMessage {
    let request = requests::AppLaunchRequest {
        request_id: 1,
        typ: MESSAGE_TYPE_LAUNCH.to_owned(),
        app_id: app_id.to_owned(),
    };

    create_message(NS_RECEIVER,
                   DEFAULT_SOURCE_ID,
                   PLATFORM_DESTINATION_ID,
                   serde_json::to_string(&request).unwrap())
}

fn get_play_media_message(session_id: &str,
                          destination: &str,
                          url: &str,
                          content_type: &str)
                          -> CastMessage {
    let request = requests::MediaRequest {
        request_id: 2,
        session_id: session_id.to_owned(),
        typ: MESSAGE_TYPE_LOAD.to_owned(),

        media: requests::Media {
            content_id: url.to_owned(),
            stream_type: STREAM_TYPE_BUFFERED.to_owned(),
            content_type: content_type.to_owned(),
        },

        current_time: 5_f64,
        autoplay: true,
        custom_data: requests::CustomData::new(),
    };



    create_message(NS_MEDIA,
                   DEFAULT_SOURCE_ID,
                   destination,
                   serde_json::to_string(&request).unwrap())
}

pub struct Chromecast {
    address: SocketAddr,
    stream: Option<SslStream<TcpStream>>,
}

impl Chromecast {
    pub fn new(host: &str, port: u16) -> Chromecast {
        let addresses: Vec<SocketAddr> = (host, port).to_socket_addrs().unwrap().collect();

        Chromecast {
            address: addresses[0],
            stream: None,
        }
    }

    pub fn get_address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn connect(&mut self) {
        let stream = TcpStream::connect(self.address).unwrap();
        let ssl_context = SslContext::new(SslMethod::Sslv23).unwrap();

        let mut stream = SslStream::connect(&ssl_context, stream).unwrap();

        self.send_message(get_connect_message(PLATFORM_DESTINATION_ID));
        self.send_message(get_ping_message());

        self.stream = Some(stream);
    }

    pub fn open_app(&self, app_id: &str) {}

    fn send_message(&mut self, message: CastMessage) {
        let serialized_message = utils::to_vec(message).unwrap();

        let mut length_buf: [u8; 4] = [0, 0, 0, 0];
        utils::write_u32_to_buffer(&mut length_buf, serialized_message.len() as u32);

        let mut stream = self.stream.as_mut().unwrap();
        stream.write(&length_buf).unwrap();
        stream.write(&serialized_message).unwrap();
    }
}



fn read_length<T>(reader: &mut T) -> u32
    where T: io::Read + Sized
{
    let mut buffer: Vec<u8> = Vec::with_capacity(4);
    let mut limited_reader = reader.take(4);
    limited_reader.read_to_end(&mut buffer).unwrap();

    utils::read_u32_from_buffer(&buffer)
}

fn read_message<T>(reader: &mut T) -> CastMessage
    where T: io::Read + Sized
{
    let length = read_length(reader);

    let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
    let mut limited_reader = reader.take(length as u64);
    limited_reader.read_to_end(&mut buffer).unwrap();

    utils::from_vec(buffer.iter().cloned().collect()).unwrap()
}

fn send_message<T>(writer: &mut T, message: CastMessage)
    where T: io::Write
{
    let serialized_message = utils::to_vec(message).unwrap();

    // ignore the Result
    let mut length_buf: [u8; 4] = [0, 0, 0, 0];
    utils::write_u32_to_buffer(&mut length_buf, serialized_message.len() as u32);

    writer.write(&length_buf).unwrap();
    writer.write(&serialized_message).unwrap();
}

fn main() {
    let address: Vec<_> = ("192.168.1.9", 8009)
                              .to_socket_addrs()
                              .unwrap()
                              .collect();



    let stream = TcpStream::connect(address[0]).unwrap();
    let ssl_context = SslContext::new(SslMethod::Sslv23).unwrap();

    let mut ssl_stream = SslStream::connect(&ssl_context, stream).unwrap();

    send_message(&mut ssl_stream,
                 get_connect_message(PLATFORM_DESTINATION_ID));
    send_message(&mut ssl_stream, get_ping_message());
    send_message(&mut ssl_stream, get_app_launch_message(APP_MEDIA_RECEIVER));

    loop {
        let message = read_message(&mut ssl_stream);

        println!("---------------------\nReceived: {:?}", message);

        match message.get_namespace() {
            NS_RECEIVER => {
                let receiver_response: requests::ReceiverResponse =
                    serde_json::from_str(message.get_payload_utf8()).unwrap();

                println!("---------------------\nPyaload {:?}", receiver_response);

                if receiver_response.status.applications.len() > 0 {
                    let application = &receiver_response.status.applications[0];

                    // Connect to application.
                    send_message(&mut ssl_stream,
                                 get_connect_message(&application.transport_id));

                    let video_url = "http://admin:271177@dcs5020l77da.local/video.cgi";

                    // Ask application to load video.
                    let play_message = get_play_media_message(&application.session_id,
                                                              &application.transport_id,
                                                              video_url,
                                                              "video/x-jpeg");
                    println!("---------------------\nPlay message {:?}", play_message);
                    send_message(&mut ssl_stream, play_message);
                    break;
                }
            }
            _ => {}
        }
    }
}