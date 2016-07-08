use std::io::{Read, Write};

use cast::cast_channel;
use utils;
use errors::Error;

/// Type of the payload that `CastMessage` can have.
#[derive(Debug)]
pub enum CastMessagePayload {
    /// Payload represented by UTF-8 string (usually it's just a JSON string).
    String(String),
    /// Payload represented by binary data.
    Binary(Vec<u8>),
}

/// Base structure that represents messages that are exchanged between Receiver and Sender.
#[derive(Debug)]
pub struct CastMessage {
    /// A namespace is a labeled protocol. That is, messages that are exchanged throughout the
    /// Cast ecosystem utilize namespaces to identify the protocol of the message being sent.
    pub namespace: String,
    /// Unique identifier of the `sender` application.
    pub source: String,
    /// Unique identifier of the `receiver` application.
    pub destination: String,
    /// Payload data attached to the message (either string or binary).
    pub payload: CastMessagePayload,
}

/// Static structure that is responsible for (de)serializing and sending/receiving Cast protocol
/// messages.
pub struct MessageManager;

impl MessageManager {
    pub fn send<W>(writer: &mut W, message: CastMessage) -> Result<(), Error> where W: Write {
        let mut raw_message = cast_channel::CastMessage::new();

        raw_message.set_protocol_version(cast_channel::CastMessage_ProtocolVersion::CASTV2_1_0);

        raw_message.set_namespace(message.namespace);
        raw_message.set_source_id(message.source);
        raw_message.set_destination_id(message.destination);

        match message.payload {
            CastMessagePayload::String(payload) => {
                raw_message.set_payload_type(cast_channel::CastMessage_PayloadType::STRING);
                raw_message.set_payload_utf8(payload);
            },

            CastMessagePayload::Binary(payload) => {
                raw_message.set_payload_type(cast_channel::CastMessage_PayloadType::BINARY);
                raw_message.set_payload_binary(payload);
            },
        };

        let message_content_buffer = try!(utils::to_vec(&raw_message));
        let message_length_buffer = try!(
            utils::write_u32_to_buffer(message_content_buffer.len() as u32));

        try!(writer.write(&message_length_buffer));
        try!(writer.write(&message_content_buffer));

        debug!("Message sent: {:?}", raw_message);

        Ok(())
    }

    pub fn receive<T>(reader: &mut T) -> Result<CastMessage, Error> where T: Read {
        let mut buffer: [u8; 4] = [0; 4];

        try!(reader.read_exact(&mut buffer));

        let length = try!(utils::read_u32_from_buffer(&buffer));

        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        let mut limited_reader = reader.take(length as u64);

        try!(limited_reader.read_to_end(&mut buffer));

        let raw_message = try!(
            utils::from_vec::<cast_channel::CastMessage>(buffer.iter().cloned().collect()));

        debug!("Message received: {:?}", raw_message);

        Ok(CastMessage {
            namespace: raw_message.get_namespace().to_owned(),
            source: raw_message.get_source_id().to_owned(),
            destination: raw_message.get_destination_id().to_owned(),
            payload: match raw_message.get_payload_type() {
                cast_channel::CastMessage_PayloadType::STRING => CastMessagePayload::String(
                    raw_message.get_payload_utf8().to_owned()),
                cast_channel::CastMessage_PayloadType::BINARY => CastMessagePayload::Binary(
                    raw_message.get_payload_binary().to_owned()),
            }
        })
    }
}
