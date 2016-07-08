use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use serde_json;

use cast::cast_channel;
use errors::Error;
use message_manager::MessageManager;

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.tp.heartbeat";

const MESSAGE_TYPE_PING: &'static str = "PING";
const MESSAGE_TYPE_PONG: &'static str = "PONG";

#[derive(Serialize, Debug)]
struct HeartBeatRequest {
    #[serde(rename="type")]
    pub typ: String,
}

#[derive(Debug)]
pub enum HeartbeatResponse {
    Ping,
    Pong,
    NotImplemented(String, serde_json::Value),
}

pub struct HeartbeatChannel<'a, W> where W: Write {
    sender: Cow<'a, str>,
    receiver: Cow<'a, str>,
    writer: Rc<RefCell<W>>,
}

impl<'a, W> HeartbeatChannel<'a, W> where W: Write {
    pub fn new<S>(sender: S, receiver: S, writer: Rc<RefCell<W>>)
        -> HeartbeatChannel<'a, W> where S: Into<Cow<'a, str>> {
        HeartbeatChannel {
            sender: sender.into(),
            receiver: receiver.into(),
            writer: writer,
        }
    }

    pub fn ping(&self) -> Result<(), Error> {
        MessageManager::send(&mut *self.writer.borrow_mut(), CHANNEL_NAMESPACE.to_owned(),
                             self.sender.to_string(), self.receiver.to_string(),
                             Some(HeartBeatRequest { typ: MESSAGE_TYPE_PING.to_owned() }))
    }

    pub fn pong(&self) -> Result<(), Error> {
        MessageManager::send(&mut *self.writer.borrow_mut(), CHANNEL_NAMESPACE.to_owned(),
                             self.sender.to_string(), self.receiver.to_string(),
                             Some(HeartBeatRequest { typ: MESSAGE_TYPE_PONG.to_owned() }))
    }

    pub fn can_handle(&self, message: &cast_channel::CastMessage) -> bool {
        message.get_namespace() == CHANNEL_NAMESPACE
    }

    pub fn parse(&self, message: &cast_channel::CastMessage) -> Result<HeartbeatResponse, Error> {
        let reply = try!(serde_json::from_str::<serde_json::Value>(message.get_payload_utf8()));

        let message_type = reply.as_object()
            .and_then(|object| object.get("type"))
            .and_then(|property| property.as_string())
            .unwrap_or("")
            .to_owned();

        let response = match message_type.as_ref() {
            MESSAGE_TYPE_PING => HeartbeatResponse::Ping,
            MESSAGE_TYPE_PONG => HeartbeatResponse::Pong,
            _ => HeartbeatResponse::NotImplemented(message_type.to_owned(), reply),
        };

        Ok(response)
    }
}
