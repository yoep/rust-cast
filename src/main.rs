#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate openssl;
extern crate protobuf;
extern crate serde;
extern crate serde_json;

mod cast_api;

use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};
use std::error::Error;
use std::mem::transmute;
use std::ptr::copy_nonoverlapping;
use std::io;

use openssl::ssl::{SslContext, SslStream, SslMethod};

use cast_api::cast_channel::*;
use protobuf::*;

const PLATFORM_DESTINATION: &'static str = "receiver-0";

#[derive(Serialize, Deserialize, Debug)]
struct ServiceRequest {
    #[serde(rename="type")]
    typ: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppLaunchRequest {
    #[serde(rename="type")]
    typ: String,
    #[serde(rename="requestId")]
    request_id: i32,
    #[serde(rename="appId")]
    app_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Media {
    #[serde(rename="contentId")]
    content_id: String,
    #[serde(rename="streamType")]
    stream_type: String,
    #[serde(rename="contentType")]
    content_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomData {
    #[serde(skip_serializing)]
    _id: (),
}

#[derive(Serialize, Deserialize, Debug)]
struct MediaRequest {
    #[serde(rename="currentTime")]
    current_time: f64,
    media: Media,
    #[serde(rename="customData")]
    custom_data: CustomData,
    #[serde(rename="sessionId")]
    session_id: String,
    #[serde(rename="requestId")]
    request_id: i32,
    #[serde(rename="type")]
    typ: String,
    autoplay: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Volume {
    level: f64,
    muted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Namespace {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Application {
    #[serde(rename="appId")]
    app_id: String,
    #[serde(rename="displayName")]
    display_name: String,
    namespaces: Vec<Namespace>,
    #[serde(rename="sessionId")]
    session_id: String,
    #[serde(rename="statusText")]
    status_text: String,
    #[serde(rename="transportId")]
    transport_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReceiverStatus {
    #[serde(default)]
    applications: Vec<Application>,
    #[serde(rename="isActiveInput")]
    is_active_input: bool,
    #[serde(rename="isStandBy")]
    is_stand_by: bool,
    volume: Volume,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReceiverMessagePayload {
    #[serde(rename="requestId")]
    request_id: i32,
    #[serde(rename="type")]
    typ: String,
    status: ReceiverStatus,
}

fn read_big_endian_u32(buf: &[u8]) -> u32 {
    unsafe { (*(buf.as_ptr() as *const u32)).to_be() }
}

fn write_big_endian_u32(buf: &mut [u8], n: u32) {
    unsafe {
        let bytes = transmute::<_, [u8; 4]>(n.to_be());
        copy_nonoverlapping((&bytes).as_ptr(), buf.as_mut_ptr(), 4);
    }
}

fn to_vec<M: protobuf::Message>(message: M) -> Result<Vec<u8>, Box<Error>> {
    let mut buf: Vec<u8> = Vec::new();
    try!(message.write_to_writer(&mut buf));
    Ok(buf)
}

/// Read a message in a buffer (Vec<u8>)
fn from_vec<M: MessageStatic>(buf: Vec<u8>) -> Result<M, Box<Error>> {
    let mut read_buf = std::io::Cursor::new(buf);
    Ok(try!(protobuf::parse_from_reader::<M>(&mut read_buf)))
}

fn create_message(namespace: &str, destination: &str, payload: String) -> CastMessage {
    let mut message = CastMessage::new();

    message.set_protocol_version(CastMessage_ProtocolVersion::CASTV2_1_0);
    message.set_source_id("sender-0".to_owned());
    message.set_destination_id(destination.to_owned());

    message.set_namespace(namespace.to_owned());
    message.set_payload_type(CastMessage_PayloadType::STRING);
    message.set_payload_utf8(payload);

    message
}

fn get_connect_message(destination: Option<&str>) -> CastMessage {
    let payload = serde_json::to_string(&ServiceRequest { typ: format!("CONNECT") });

    create_message("urn:x-cast:com.google.cast.tp.connection",
                   destination.unwrap_or(PLATFORM_DESTINATION),
                   payload.unwrap())
}

fn get_ping_message() -> CastMessage {
    let payload = serde_json::to_string(&ServiceRequest { typ: format!("PING") });

    create_message("urn:x-cast:com.google.cast.tp.heartbeat",
                   PLATFORM_DESTINATION,
                   payload.unwrap())
}

fn get_app_launch_message(app_id: String) -> CastMessage {
    let payload = serde_json::to_string(&AppLaunchRequest {
        typ: format!("LAUNCH"),
        app_id: app_id,
        request_id: 1,
    });

    create_message("urn:x-cast:com.google.cast.receiver",
                   PLATFORM_DESTINATION,
                   payload.unwrap())
}

fn get_play_media_message(session_id: &str, destination: &str, url: String) -> CastMessage {
    let payload = serde_json::to_string(&MediaRequest {
        typ: format!("LOAD"),
        media: Media {
            content_id: url,
            stream_type: format!("BUFFERED"),
            content_type: format!("video/mp4"),
        },
        current_time: 0_f64,
        autoplay: true,
        custom_data: CustomData { _id: () },
        request_id: 1,
        session_id: session_id.to_owned(),
    });

    create_message("urn:x-cast:com.google.cast.media",
                   destination,
                   payload.unwrap())
}

fn read_length<T>(reader: &mut T) -> u32
    where T: io::Read + Sized
{
    let mut buffer: Vec<u8> = Vec::with_capacity(4);
    let mut limited_reader = reader.take(4);
    limited_reader.read_to_end(&mut buffer).unwrap();

    read_big_endian_u32(&buffer)
}

fn read_message<T>(reader: &mut T) -> CastMessage
    where T: io::Read + Sized
{
    let length = read_length(reader);

    let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
    let mut limited_reader = reader.take(length as u64);
    limited_reader.read_to_end(&mut buffer).unwrap();

    from_vec(buffer.iter().cloned().collect()).unwrap()
}

fn send_message<T>(writer: &mut T, message: CastMessage)
    where T: io::Write
{
    let serialized_message = to_vec(message).unwrap();

    // ignore the Result
    let mut length_buf: [u8; 4] = [0, 0, 0, 0];
    write_big_endian_u32(&mut length_buf, serialized_message.len() as u32);

    writer.write(&length_buf).unwrap();
    writer.write(&serialized_message).unwrap();
}

fn main() {
    let address: Vec<_> = ("192.168.1.18", 8009)
                              .to_socket_addrs()
                              .unwrap()
                              .collect();

    let stream = TcpStream::connect(address[0]).unwrap();
    let ssl_context = SslContext::new(SslMethod::Sslv23).unwrap();

    let mut ssl_stream = SslStream::connect(&ssl_context, stream).unwrap();

    println!("Stream opened!!!");

    send_message(&mut ssl_stream, get_connect_message(None));
    send_message(&mut ssl_stream, get_ping_message());
    // send_message(&mut ssl_stream, get_app_launch_message(format!("YouTube")));
    send_message(&mut ssl_stream, get_app_launch_message(format!("CC1AD845")));

    println!("Sent message");

    let mut i = 0;

    loop {
        let message = read_message(&mut ssl_stream);

        println!("-----\nDeserialized message: {:?}", message);

        match message.get_namespace() {
            "urn:x-cast:com.google.cast.receiver" => {
                let payload_str = message.get_payload_utf8();
                let payload: ReceiverMessagePayload = serde_json::from_str(payload_str).unwrap();

                println!("-----\nPyaload {:?}", payload);

                if payload.status.applications.len() > 0 {
                    send_message(&mut ssl_stream, get_connect_message(Some(&payload.status.applications[0]
                                                                   .transport_id)));

                    let play_message = get_play_media_message(&payload.status.applications[0]
                                                                   .session_id,
                                                              &payload.status.applications[0]
                                                                   .transport_id,
                                                              format!("http://commondatastorage.\
                                                                       googleapis.\
                                                                       com/gtv-videos-bucket/sam\
                                                                       ple/BigBuckBunny.mp4"));
                    println!("-----\nPlay message {:?}", play_message);
                    send_message(&mut ssl_stream, play_message);


                }
            }
            _ => {}
        }

        i = i + 1;
    }
}