use std::io::{Read, Write};

use serde;
use serde_json;

use cast::cast_channel;
use utils;
use errors::Error;

pub struct MessageManager;

impl MessageManager {
    pub fn send<W>(writer: &mut W, message: cast_channel::CastMessage) -> Result<(), Error>
        where W: Write
    {
        let message_content_buffer = utils::to_vec(message).unwrap();

        let message_length_buffer = try!(
            utils::write_u32_to_buffer(message_content_buffer.len() as u32));

        try!(writer.write(&message_length_buffer));
        try!(writer.write(&message_content_buffer));

        Ok(())
    }

    pub fn receive<T>(reader: &mut T) -> cast_channel::CastMessage
        where T: Read
    {
        let length = MessageManager::receive_length(reader).unwrap();

        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        let mut limited_reader = reader.take(length as u64);
        limited_reader.read_to_end(&mut buffer).unwrap();

        utils::from_vec(buffer.iter().cloned().collect()).unwrap()
    }

    pub fn create<P>(namespace: String,
                     sender: String,
                     receiver: String,
                     payload: Option<P>)
                     -> cast_channel::CastMessage
        where P: serde::Serialize
    {
        let mut message = cast_channel::CastMessage::new();

        message.set_protocol_version(cast_channel::CastMessage_ProtocolVersion::CASTV2_1_0);

        message.set_namespace(namespace);
        message.set_source_id(sender);
        message.set_destination_id(receiver);

        if payload.is_some() {
            message.set_payload_type(cast_channel::CastMessage_PayloadType::STRING);
            message.set_payload_utf8(serde_json::to_string(&payload.unwrap()).unwrap());
        }

        message
    }

    pub fn parse_payload<P>(message: &cast_channel::CastMessage) -> Result<P, Error>
        where P: serde::Deserialize
    {
        Ok(try!(serde_json::from_str(message.get_payload_utf8())))
    }

    fn receive_length<T>(reader: &mut T) -> Result<u32, Error> where T: Read
    {
        let mut buffer: [u8; 4] = [0; 4];
        try!(reader.read_exact(&mut buffer));

        utils::read_u32_from_buffer(&buffer)
    }
}
