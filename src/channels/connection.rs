use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use serde_json::Value;

use cast::cast_channel;
use errors::Error;
use message_manager::MessageManager;

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.tp.connection";
const CHANNEL_USER_AGENT: &'static str = "RustCast";

const MESSAGE_TYPE_CONNECT: &'static str = "CONNECT";
const MESSAGE_TYPE_CLOSE: &'static str = "CLOSE";

#[derive(Serialize, Debug)]
struct ConnectionRequest {
    #[serde(rename="type")]
    pub typ: String,
    #[serde(rename="userAgent")]
    pub user_agent: String,
}

#[derive(Debug)]
pub enum ConnectionResponse {
    Connect,
    Close,
    NotImplemented(String, Value),
}

pub struct ConnectionChannel<'a, W> where W: Write {
    sender: Cow<'a, str>,
    writer: Rc<RefCell<W>>,
}

impl<'a, W> ConnectionChannel<'a, W> where W: Write {
    pub fn new<S>(sender: S, writer: Rc<RefCell<W>>)
        -> ConnectionChannel<'a, W> where S: Into<Cow<'a, str>> {
        ConnectionChannel {
            sender: sender.into(),
            writer: writer,
        }
    }

    pub fn connect<S>(&self, destination: S) -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = ConnectionRequest {
            typ: MESSAGE_TYPE_CONNECT.to_owned(),
            user_agent: CHANNEL_USER_AGENT.to_owned(),
        };

        MessageManager::send(&mut *self.writer.borrow_mut(), CHANNEL_NAMESPACE.to_owned(),
                             self.sender.to_string(), destination.into().to_string(), Some(payload))
    }

    pub fn disconnect<S>(&self, destination: S) -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = ConnectionRequest {
            typ: MESSAGE_TYPE_CLOSE.to_owned(),
            user_agent: CHANNEL_USER_AGENT.to_owned(),
        };

        MessageManager::send(&mut *self.writer.borrow_mut(), CHANNEL_NAMESPACE.to_owned(),
                             self.sender.to_string(), destination.into().to_string(), Some(payload))
    }

    pub fn can_handle(&self, message: &cast_channel::CastMessage) -> bool {
        message.get_namespace() == CHANNEL_NAMESPACE
    }

    pub fn parse(&self, message: &cast_channel::CastMessage) -> Result<ConnectionResponse, Error> {
        let reply: Value = try!(MessageManager::parse_payload(message));

        let message_type = reply.as_object()
            .and_then(|object| object.get("type"))
            .and_then(|property| property.as_string())
            .unwrap_or("")
            .to_owned();

        let response = match message_type.as_ref() {
            MESSAGE_TYPE_CONNECT => ConnectionResponse::Connect,
            MESSAGE_TYPE_CLOSE => ConnectionResponse::Close,
            _ => ConnectionResponse::NotImplemented(message_type.to_owned(), reply),
        };

        Ok(response)
    }
}
