use std::time::Duration;
use std::{
    io::{Read, Write},
    num::NonZeroU32,
    ops::{Deref, DerefMut},
    thread,
};

use crossbeam_channel::{select, tick, Receiver, Sender};
use tokio_util::sync::CancellationToken;

use crate::{
    cast::{
        cast_channel,
        cast_channel::cast_message::{PayloadType, ProtocolVersion},
    },
    errors::Error,
    utils,
};

const DEFAULT_MESSAGE_TIMEOUT_SECONDS: u64 = 30;

struct Lock<T>(
    #[cfg(feature = "thread_safe")] std::sync::Mutex<T>,
    #[cfg(not(feature = "thread_safe"))] std::cell::RefCell<T>,
);

struct LockGuardMut<'a, T>(
    #[cfg(feature = "thread_safe")] std::sync::MutexGuard<'a, T>,
    #[cfg(not(feature = "thread_safe"))] std::cell::RefMut<'a, T>,
);

impl<'a, T> Deref for LockGuardMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a, T> DerefMut for LockGuardMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl<T> Lock<T> {
    fn new(data: T) -> Self {
        Lock({
            #[cfg(feature = "thread_safe")]
            let lock = std::sync::Mutex::new(data);
            #[cfg(not(feature = "thread_safe"))]
            let lock = std::cell::RefCell::new(data);
            lock
        })
    }

    fn borrow_mut(&self) -> LockGuardMut<'_, T> {
        LockGuardMut({
            #[cfg(feature = "thread_safe")]
            let guard = self.0.lock().unwrap();
            #[cfg(not(feature = "thread_safe"))]
            let guard = self.0.borrow_mut();
            guard
        })
    }
}

/// Type of the payload that `CastMessage` can have.
#[derive(Debug, Clone, PartialEq)]
pub enum CastMessagePayload {
    /// Payload represented by UTF-8 string (usually it's just a JSON string).
    String(String),
    /// Payload represented by binary data.
    Binary(Vec<u8>),
}

/// Base structure that represents messages that are exchanged between Receiver and Sender.
#[derive(Debug, Clone, PartialEq)]
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

impl From<cast_channel::CastMessage> for CastMessage {
    fn from(value: cast_channel::CastMessage) -> Self {
        Self {
            namespace: value.namespace().to_string(),
            source: value.source_id().to_string(),
            destination: value.destination_id().to_string(),
            payload: match value.payload_type() {
                PayloadType::STRING => CastMessagePayload::String(value.payload_utf8().to_string()),
                PayloadType::BINARY => {
                    CastMessagePayload::Binary(value.payload_binary().to_owned())
                }
            },
        }
    }
}

/// Static structure that is responsible for (de)serializing and sending/receiving Cast protocol
/// messages.
pub struct MessageManager<W>
where
    W: Write,
{
    stream_writer: Lock<W>,
    request_counter: Lock<NonZeroU32>,
    channel_receiver: Receiver<CastMessage>,
    cancellation_token: CancellationToken,
}

impl<W> MessageManager<W>
where
    W: Write,
{
    pub fn new<R: Read + Send + 'static>(mut reader: R, writer: W) -> Self {
        // use a message channel to receive messages from the stream
        // this allows us to fanout messages to multiple consumers/receivers without stealing messages
        // from one of them
        let (sender, receiver) = crossbeam_channel::unbounded();
        let cancellation_token = CancellationToken::new();

        let thread_cancel = cancellation_token.clone();
        thread::spawn(move || {
            log::trace!("Starting to poll for new messages");
            loop {
                if thread_cancel.is_cancelled() {
                    break;
                }

                if let Err(e) = Self::poll_message(&mut reader, &sender) {
                    log::error!("Failed to poll Chromecast message, {}", e);
                }
            }
            log::debug!("Messages poller has been stopped");
        });

        MessageManager {
            stream_writer: Lock::new(writer),
            request_counter: Lock::new(NonZeroU32::MIN),
            channel_receiver: receiver,
            cancellation_token,
        }
    }

    /// Sends `message` to the Cast Device.
    ///
    /// # Arguments
    ///
    /// * `message` - `CastMessage` instance to be sent to the Cast Device.
    pub fn send(&self, message: CastMessage) -> Result<(), Error> {
        let mut raw_message = cast_channel::CastMessage::new();

        raw_message.set_protocol_version(ProtocolVersion::CASTV2_1_0);

        raw_message.set_namespace(message.namespace);
        raw_message.set_source_id(message.source);
        raw_message.set_destination_id(message.destination);

        match message.payload {
            CastMessagePayload::String(payload) => {
                raw_message.set_payload_type(PayloadType::STRING);
                raw_message.set_payload_utf8(payload);
            }

            CastMessagePayload::Binary(payload) => {
                raw_message.set_payload_type(PayloadType::BINARY);
                raw_message.set_payload_binary(payload);
            }
        };

        let message_content_buffer = utils::to_vec(&raw_message)?;
        let message_length_buffer =
            utils::write_u32_to_buffer(message_content_buffer.len() as u32)?;

        let writer = &mut *self.stream_writer.borrow_mut();

        writer.write_all(&message_length_buffer)?;
        writer.write_all(&message_content_buffer)?;

        log::debug!("Message sent: {:?}", raw_message);

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
        self.channel_receiver
            .recv()
            .map_err(|e| Error::Internal(e.to_string()))
    }

    /// Generates unique integer number that is used in some requests to map them with the response.
    ///
    /// # Return value
    ///
    /// Unique (in the scope of this particular `MessageManager` instance) integer number.
    pub fn generate_request_id(&self) -> NonZeroU32 {
        let mut counter = self.request_counter.borrow_mut();
        let request_id = *counter;
        *counter = counter.checked_add(1).unwrap();
        request_id
    }

    /// Subscribe to the message channel to receive incoming messages from the Chromecast device.
    ///
    /// # Note
    ///
    /// If a receiver is no longer used, it should be **dropped** to avoid the message channel from
    /// storing infinite messages.
    ///
    /// # Return value
    ///
    /// Returns a receiver channel to receive incoming messages from the Chromecast device.
    pub fn subscribe(&self) -> Receiver<CastMessage> {
        self.channel_receiver.clone()
    }

    /// Subscribe to the message channel to receive incoming messages from the Chromecast device.
    /// The incoming messages are mapped using the provided `mapping_fn` function and filtered using
    /// the provided mapping logic.
    ///
    /// # Note
    ///
    /// In regard to the `subscribe` fn, the receiver is automatically dropped once a message is received
    /// or an error occurred while processing a message.
    ///
    /// # Return value
    ///
    /// Returns the mapped message if found or an error if there was a problem with the mapping.
    pub fn subscribe_find<F, B>(&self, mapping_fn: F) -> Result<B, Error>
    where
        F: Fn(&CastMessage) -> Result<Option<B>, Error>,
    {
        let receiver = self.subscribe();
        let timeout = tick(Duration::from_secs(DEFAULT_MESSAGE_TIMEOUT_SECONDS));

        loop {
            select! {
                recv(receiver) -> message => {
                    let message = message.map_err(|e| Error::Internal(e.to_string()))?;
                    match mapping_fn(&message)? {
                        Some(r) => return Ok(r),
                        None => {}
                    }
                }
                recv(timeout) -> _ => {
                    return Err(Error::Timeout("Timed out while waiting for message".to_string()));
                }
            }
        }
    }

    fn poll_message<R: Read + Send>(
        reader: &mut R,
        channel_sender: &Sender<CastMessage>,
    ) -> Result<(), Error> {
        log::trace!("Trying to read the next message length from the stream");
        let mut buffer: [u8; 4] = [0; 4];
        reader.read_exact(&mut buffer)?;

        let length = utils::read_u32_from_buffer(&buffer)?;
        log::trace!("Next message stream length is {}", length);

        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        let mut limited_reader = reader.take(u64::from(length));

        log::trace!("Trying to read the next message from the stream");
        limited_reader.read_to_end(&mut buffer)?;

        let raw_message = utils::from_vec::<cast_channel::CastMessage>(buffer.to_vec())?;

        log::debug!("Message received: {:?}", raw_message);
        let message = CastMessage::from(raw_message);

        // use the channel sender to send the received message
        channel_sender
            .send(message)
            .map_err(|e| Error::Internal(format!("failed to send channel message, {}", e)))?;
        Ok(())
    }
}

impl<W> Drop for MessageManager<W>
where
    W: Write,
{
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}

#[cfg(test)]
mod tests {
    use protobuf::EnumOrUnknown;

    use crate::tests::{init_logger, MockTcpStream};
    use crate::{DEFAULT_RECEIVER_ID, DEFAULT_SENDER_ID};

    use super::*;

    #[test]
    fn test_receive() {
        init_logger();
        let mut stream = MockTcpStream::new();
        let payload = r#"{"type":"PING"}"#;
        stream.add_message(cast_channel::CastMessage {
            protocol_version: Some(EnumOrUnknown::new(ProtocolVersion::CASTV2_1_2)),
            source_id: Some(DEFAULT_RECEIVER_ID.to_string()),
            destination_id: Some(DEFAULT_SENDER_ID.to_string()),
            namespace: Some(crate::channels::heartbeat::CHANNEL_NAMESPACE.to_string()),
            payload_type: Some(EnumOrUnknown::new(PayloadType::STRING)),
            payload_utf8: Some(payload.to_string()),
            payload_binary: None,
            continued: None,
            remaining_length: None,
            special_fields: Default::default(),
        });
        let (reader, writer) = stream.split();
        let message_manager = MessageManager::new(reader, writer);
        let expected_result = CastMessage {
            namespace: crate::channels::heartbeat::CHANNEL_NAMESPACE.to_string(),
            source: DEFAULT_RECEIVER_ID.to_string(),
            destination: DEFAULT_SENDER_ID.to_string(),
            payload: CastMessagePayload::String(payload.to_string()),
        };

        let result = message_manager
            .receive()
            .expect("expected to receive a message");

        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_send() {
        init_logger();
        let payload = r#"{"type":"PONG"}"#;
        let namespace = crate::channels::heartbeat::CHANNEL_NAMESPACE;
        let stream = MockTcpStream::new();
        let (reader, writer) = stream.split();
        let message_manager = MessageManager::new(reader, writer);
        let expected_message = cast_channel::CastMessage {
            protocol_version: Some(EnumOrUnknown::new(ProtocolVersion::CASTV2_1_0)),
            source_id: Some(DEFAULT_SENDER_ID.to_string()),
            destination_id: Some(DEFAULT_RECEIVER_ID.to_string()),
            namespace: Some(namespace.to_string()),
            payload_type: Some(EnumOrUnknown::new(PayloadType::STRING)),
            payload_utf8: Some(payload.to_string()),
            payload_binary: None,
            continued: None,
            remaining_length: None,
            special_fields: Default::default(),
        };

        message_manager
            .send(CastMessage {
                namespace: namespace.to_string(),
                source: DEFAULT_SENDER_ID.to_string(),
                destination: DEFAULT_RECEIVER_ID.to_string(),
                payload: CastMessagePayload::String(payload.to_string()),
            })
            .unwrap();

        let tcp_message = stream
            .received_message(0)
            .expect("expected a message to have been received");
        assert_eq!(expected_message, tcp_message.message());
    }
}
