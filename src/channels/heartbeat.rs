use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

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

#[derive(Deserialize, Debug)]
pub struct HeartbeatResponse {
    #[serde(rename="type")]
    pub typ: String,
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
        let payload = HeartBeatRequest {
            typ: MESSAGE_TYPE_PING.to_owned(),
        };

        let message = try!(MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                                  self.sender.to_string(),
                                                  self.receiver.to_string(),
                                                  Some(payload)));

        MessageManager::send(&mut *self.writer.borrow_mut(), message)
    }

    pub fn pong(&self) -> Result<(), Error> {
        let payload = HeartBeatRequest {
            typ: MESSAGE_TYPE_PONG.to_owned(),
        };

        let message = try!(MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                                  self.sender.to_string(),
                                                  self.receiver.to_string(),
                                                  Some(payload)));

        MessageManager::send(&mut *self.writer.borrow_mut(), message)
    }

    pub fn can_handle(&self, message: &cast_channel::CastMessage) -> bool {
        message.get_namespace() == CHANNEL_NAMESPACE
    }

    pub fn parse(&self, message: &cast_channel::CastMessage) -> Result<HeartbeatResponse, Error> {
        MessageManager::parse_payload(message)
    }
}
