extern crate protobuf;

mod cast_api;

use std::io::prelude::*;
use std::net::TcpStream;
use std::error::Error;

use cast_api::cast_channel::*;
use protobuf::*;


fn to_vec<M: protobuf::Message>(message: M) -> Result<Vec<u8>, Box<Error>> {
    let mut buf: Vec<u8> = Vec::new();
    try!(message.write_to_writer(&mut buf));
    Ok(buf)
}

fn main() {
    let mut stream = TcpStream::connect("192.168.1.14:8009").unwrap();

    println!("Stream opened!!!");

    let mut message = CastMessage::new();

    message.set_protocol_version(CastMessage_ProtocolVersion::CASTV2_1_0);
    message.set_source_id("sender-0".to_owned());
    message.set_destination_id("receiver-0".to_owned());

    message.set_namespace("urn:x-cast:com.google.cast.tp.connection".to_owned());
    message.set_payload_type(CastMessage_PayloadType::STRING);
    message.set_payload_utf8("".to_owned());

    let serialized_message = to_vec(message).unwrap();


    // msg.payload_utf8 = _json_to_payload(data)

    // ignore the Result
    let _ = stream.write(&serialized_message);
    // let _ = stream.read(&mut [0; 128]); // ignore here too

    println!("Opened Connection!!!");

    loop {}
}