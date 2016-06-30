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

pub struct HeartbeatChannel<W>
    where W: Write
{
    sender: String,
    receiver: String,
    writer: Rc<RefCell<W>>,
}

impl<W> HeartbeatChannel<W>
    where W: Write
{
    pub fn new(sender: String, receiver: String, writer: Rc<RefCell<W>>) -> HeartbeatChannel<W> {
        HeartbeatChannel {
            sender: sender,
            receiver: receiver,
            writer: writer,
        }
    }

    pub fn try_handle(&self, message: &cast_channel::CastMessage) -> Result<HeartbeatResponse, Error> {
        if message.get_namespace() != CHANNEL_NAMESPACE {
            return Err(Error::Internal("Channel does not support provided message.".to_owned()));
        }

        MessageManager::parse_payload(message)
    }

    pub fn ping(&self) -> Result<(), Error> {
        let payload = HeartBeatRequest {
            typ: MESSAGE_TYPE_PING.to_owned(),
        };

        let message = try!(MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                                  self.sender.clone(),
                                                  self.receiver.clone(),
                                                  Some(payload)));

        MessageManager::send(&mut *self.writer.borrow_mut(), message)
    }

    pub fn pong(&self) -> Result<(), Error> {
        let payload = HeartBeatRequest {
            typ: MESSAGE_TYPE_PONG.to_owned(),
        };

        let message = try!(MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                                  self.sender.clone(),
                                                  self.receiver.clone(),
                                                  Some(payload)));

        MessageManager::send(&mut *self.writer.borrow_mut(), message)
    }
}
