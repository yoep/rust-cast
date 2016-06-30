use std::io::{Read, Write};

use serde;
use serde_json;

use cast::cast_channel;
use utils;
use errors::Error;

pub struct MessageManager;

impl MessageManager {
    pub fn send<W>(writer: &mut W, message: cast_channel::CastMessage)
        -> Result<(), Error> where W: Write
    {
        let message_content_buffer = try!(utils::to_vec(&message));
        let message_length_buffer = try!(
            utils::write_u32_to_buffer(message_content_buffer.len() as u32));

        try!(writer.write(&message_length_buffer));
        try!(writer.write(&message_content_buffer));

        debug!("Message sent: {:?}", message);

        Ok(())
    }

    pub fn receive<T>(reader: &mut T) -> Result<cast_channel::CastMessage, Error> where T: Read
    {
        let length = try!(MessageManager::receive_length(reader));

        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        let mut limited_reader = reader.take(length as u64);

        try!(limited_reader.read_to_end(&mut buffer));

        let message = try!(utils::from_vec(buffer.iter().cloned().collect()));

        debug!("Message received: {:?}", message);

        Ok(message)
    }

    pub fn create<P>(namespace: String, sender: String, receiver: String, payload: Option<P>)
        -> Result<cast_channel::CastMessage, Error> where P: serde::Serialize
    {
        let mut message = cast_channel::CastMessage::new();

        message.set_protocol_version(cast_channel::CastMessage_ProtocolVersion::CASTV2_1_0);

        message.set_namespace(namespace);
        message.set_source_id(sender);
        message.set_destination_id(receiver);

        if payload.is_some() {
            message.set_payload_type(cast_channel::CastMessage_PayloadType::STRING);
            message.set_payload_utf8(try!(serde_json::to_string(&payload.unwrap())));
        }

        Ok(message)
    }

    pub fn parse_payload<P>(message: &cast_channel::CastMessage)
        -> Result<P, Error> where P: serde::Deserialize
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
