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

pub struct ConnectionChannel<W>
    where W: Write
{
    sender: String,
    writer: Rc<RefCell<W>>,
}

impl<W> ConnectionChannel<W>
    where W: Write
{
    pub fn new(sender: String, writer: Rc<RefCell<W>>) -> ConnectionChannel<W> {
        ConnectionChannel {
            sender: sender,
            writer: writer,
        }
    }

    pub fn connect(&self, destination: String) {
        let message = MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                             self.sender.clone(),
                                             destination,
                                             Some(ConnectionRequest {
                                                 typ: MESSAGE_TYPE_CONNECT.to_owned(),
                                                 user_agent: CHANNEL_USER_AGENT.to_owned(),
                                             }));
        MessageManager::send(&mut *self.writer.borrow_mut(), message);
    }

    pub fn disconnect(&self, destination: String) {
        let message = MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                             self.sender.clone(),
                                             destination,
                                             Some(ConnectionRequest {
                                                 typ: MESSAGE_TYPE_CLOSE.to_owned(),
                                                 user_agent: CHANNEL_USER_AGENT.to_owned(),
                                             }));
        MessageManager::send(&mut *self.writer.borrow_mut(), message);
    }

    pub fn try_handle(&self,
                      message: &cast_channel::CastMessage)
                      -> Result<ConnectionResponse, Error> {
        if message.get_namespace() != CHANNEL_NAMESPACE {
            return Err(Error::Internal("Channel does not support provided message.".to_owned()));
        }

        MessageManager::parse_payload(message)
    }
}
