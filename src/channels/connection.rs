use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use cast::cast_channel;
use errors::Error;
use message_manager::MessageManager;

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.tp.connection";
const CHANNEL_USER_AGENT: &'static str = "ChromecastLink";

const MESSAGE_TYPE_CONNECT: &'static str = "CONNECT";
const MESSAGE_TYPE_CLOSE: &'static str = "CLOSE";

#[derive(Serialize, Debug)]
struct ConnectionRequest {
    #[serde(rename="type")]
    pub typ: String,
    #[serde(rename="userAgent")]
    pub user_agent: String,
}

#[derive(Deserialize, Debug)]
pub struct ConnectionResponse {
    #[serde(rename="type")]
    pub typ: String,
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

        let message = try!(MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                                  self.sender.to_string(),
                                                  destination.into().to_string(),
                                                  Some(payload)));

        MessageManager::send(&mut *self.writer.borrow_mut(), message)
    }

    pub fn disconnect<S>(&self, destination: S) -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = ConnectionRequest {
            typ: MESSAGE_TYPE_CLOSE.to_owned(),
            user_agent: CHANNEL_USER_AGENT.to_owned(),
        };

        let message = try!(MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                                  self.sender.to_string(),
                                                  destination.into().to_string(),
                                                  Some(payload)));

        MessageManager::send(&mut *self.writer.borrow_mut(), message)
    }

    pub fn can_handle(&self, message: &cast_channel::CastMessage) -> bool {
        message.get_namespace() == CHANNEL_NAMESPACE
    }

    pub fn parse(&self, message: &cast_channel::CastMessage) -> Result<ConnectionResponse, Error> {
        MessageManager::parse_payload(message)
    }
}
