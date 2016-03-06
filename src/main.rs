#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate openssl;
extern crate protobuf;
extern crate serde;
extern crate serde_json;

mod cast_api;

use std::io::prelude::*;
use std::net::{TcpStream, TcpListener, ToSocketAddrs};
use std::error::Error;
use std::mem::transmute;
use std::ptr::copy_nonoverlapping;

use openssl::ssl::{SslContext, SslStream, SslMethod};

use cast_api::cast_channel::*;
use protobuf::*;

#[derive(Serialize, Deserialize, Debug)]
struct StatusRequest {
    #[serde(rename="type")]
    typ: String,
    #[serde(rename="requestId")]
    request_id: i32,
}

// macro_rules! read_num_bytes {
//     ($ty:ty, $size:expr, $src:expr, $which:ident) => ({
//         assert!($size <= $src.len());
//         unsafe {
//             (*($src.as_ptr() as *const $ty)).$which()
//         }
//     });
// }

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

fn get_connect_message() -> CastMessage {
    let mut message = CastMessage::new();

    message.set_protocol_version(CastMessage_ProtocolVersion::CASTV2_1_0);
    message.set_source_id("sender-0".to_owned());
    message.set_destination_id("receiver-0".to_owned());

    message.set_namespace("urn:x-cast:com.google.cast.tp.connection".to_owned());
    message.set_payload_type(CastMessage_PayloadType::STRING);

    let request = StatusRequest {
        typ: format!("CONNECT"),
        request_id: 1,
    };

    message.set_payload_utf8(serde_json::to_string(&request).unwrap());

    message
}

fn main() {
    let address: Vec<_> = ("192.168.1.17", 8009)
                              .to_socket_addrs()
                              .unwrap()
                              .collect();

    let mut stream = TcpStream::connect(address[0]).unwrap();
    let ssl_context = SslContext::new(SslMethod::Sslv23).unwrap();

    let mut ssl_stream = SslStream::connect(&ssl_context, stream).unwrap();

    println!("Stream opened!!!");

    let message = get_connect_message();

    let serialized_message = to_vec(message).unwrap();

    // ignore the Result
    let mut length_buf: [u8; 4] = [0, 0, 0, 0];
    write_big_endian_u32(&mut length_buf, serialized_message.len() as u32);

    ssl_stream.write(&length_buf).unwrap();
    ssl_stream.write(&serialized_message).unwrap();

    println!("Sent message");

    let mut length_buf = [0; 4];

    // read exactly 10 bytes
    ssl_stream.read_exact(&mut length_buf).unwrap();

    println!("Received message length {:?} {:?}",
             length_buf,
             read_big_endian_u32(&length_buf));

    let mut message_serialized = [0; 88];
    ssl_stream.read_exact(&mut message_serialized).unwrap();

    println!("Received message {:?}", message_serialized.len());

    let p: CastMessage = from_vec(message_serialized.iter().cloned().collect()).unwrap();

    println!("Deserialized message: {:?}", p);

    loop {}
}