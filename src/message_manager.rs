use std::cell::RefCell;
use std::io::{Read, Write};

use cast::cast_channel;
use utils;
use errors::Error;

/// Type of the payload that `CastMessage` can have.
#[derive(Debug, Clone)]
pub enum CastMessagePayload {
    /// Payload represented by UTF-8 string (usually it's just a JSON string).
    String(String),
    /// Payload represented by binary data.
    Binary(Vec<u8>),
}

/// Base structure that represents messages that are exchanged between Receiver and Sender.
#[derive(Debug, Clone)]
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
pub struct MessageManager<S> where S: Write + Read {
    message_buffer: RefCell<Vec<CastMessage>>,
    stream: RefCell<S>,
    request_conter: RefCell<i32>,
}

impl<S> MessageManager<S> where S: Write + Read {
    pub fn new(stream: S) -> Self {
        MessageManager {
            stream: RefCell::new(stream),
            message_buffer: RefCell::new(vec![]),
            request_conter: RefCell::new(1),
        }
    }

    /// Sends `message` to the Cast Device.
    ///
    /// # Arguments
    ///
    /// * `message` - `CastMessage` instance to be sent to the Cast Device.
    pub fn send(&self, message: CastMessage) -> Result<(), Error> {
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

        let mut writer = &mut *self.stream.borrow_mut();

        try!(writer.write(&message_length_buffer));
        try!(writer.write(&message_content_buffer));

        debug!("Message sent: {:?}", raw_message);

        Ok(())
    }

    /// Waits for the next `CastMessage` available. Can also return existing message from the
    /// internal message buffer containing messages that have been received previously, but haven't
    /// been consumed for some reason (e.g. during `receive_find_map` call).
    ///
    /// # Return value
    ///
    /// `Result` containing parsed `CastMessage` or `Error`.
    pub fn receive(&self) -> Result<CastMessage, Error> {
        let mut message_buffer = self.message_buffer.borrow_mut();

        // If we have messages in the buffer, let's return them from it.
        match message_buffer.is_empty() {
            false => Ok(message_buffer.remove(0)),
            true => self.read(),
        }
    }

    /// Waits for the next `CastMessage` for which `f` returns valid mapped value. Messages in which
    /// `f` is not interested are placed into internal message buffer and can be later retrieved
    /// with `receive`. This method always reads from the stream.
    ///
    /// # Example
    ///
    /// ```
    /// message_manager.receive_find_map(|message| {
    ///     if !can_handle(message) {
    ///         return Ok(None);
    ///     }
    ///
    ///     parse(message)
    /// })
    /// ```
    ///
    /// # Arguments
    ///
    /// * `f` - Function that analyzes and maps `CastMessage` to any other type. If message doesn't
    /// look like something `f` is looking for, then `Ok(None)` should be returned so that message
    /// is not lost and placed into internal message buffer for later retrieval.
    ///
    /// # Return value
    ///
    /// `Result` containing parsed `CastMessage` or `Error`.
    pub fn receive_find_map<F, B>(&self, f: F)
        -> Result<B, Error> where F: Fn(&CastMessage) -> Result<Option<B>, Error> {
        loop {
            let message = try!(self.read());

            // If message is found, just return mapped result, otherwise keep unprocessed message
            // in the buffer, it can be later retrieved with `receive`.
            match try!(f(&message)) {
                Some(r) => return Ok(r),
                None => self.message_buffer.borrow_mut().push(message)
            }
        }
    }

    /// Generates unique integer number that is used in some requests to map them with the response.
    ///
    /// # Return value
    ///
    /// Unique (in the scope of this particular `MessageManager` instance) integer number.
    pub fn generate_request_id(&self) -> i32 {
        let request_id = self.request_conter.borrow().clone() + 1;

        *self.request_conter.borrow_mut() = request_id;

        request_id
    }

    /// Reads next `CastMessage` from the stream.
    ///
    /// # Return value
    ///
    /// `Result` containing parsed `CastMessage` or `Error`.
    fn read(&self) -> Result<CastMessage, Error> {
        let mut buffer: [u8; 4] = [0; 4];

        let mut reader = &mut *self.stream.borrow_mut();

        try!(reader.read_exact(&mut buffer));

        let length = try!(utils::read_u32_from_buffer(&buffer));

        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        let mut limited_reader = reader.take(length as u64);

        try!(limited_reader.read_to_end(&mut buffer));

        let raw_message = try!(
            utils::from_vec::<cast_channel::CastMessage>(buffer.iter().cloned().collect()));

        debug!("Message received: {:?}", raw_message);

        Ok(CastMessage {
            namespace: raw_message.get_namespace().to_string(),
            source: raw_message.get_source_id().to_string(),
            destination: raw_message.get_destination_id().to_string(),
            payload: match raw_message.get_payload_type() {
                cast_channel::CastMessage_PayloadType::STRING => CastMessagePayload::String(
                    raw_message.get_payload_utf8().to_string()),
                cast_channel::CastMessage_PayloadType::BINARY => CastMessagePayload::Binary(
                    raw_message.get_payload_binary().to_owned()),
            }
        })
    }
}
