use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use message_manager::MessageManager;

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.receiver";

const MESSAGE_TYPE_LAUNCH: &'static str = "LAUNCH";
const MESSAGE_TYPE_STOP: &'static str = "STOP";
const MESSAGE_TYPE_GET_STATUS: &'static str = "GET_STATUS";

#[derive(Serialize, Debug)]
struct AppLaunchRequest {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="type")]
    pub typ: String,

    #[serde(rename="appId")]
    pub app_id: String,
}

#[derive(Serialize, Debug)]
struct AppStopRequest {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="type")]
    pub typ: String,

    #[serde(rename="sessionId")]
    pub session_id: String,
}

#[derive(Serialize, Debug)]
struct GetStatusRequest {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="type")]
    pub typ: String,
}

pub struct ReceiverChannel<W>
    where W: Write
{
    sender: String,
    receiver: String,
    writer: Rc<RefCell<W>>,
}

impl<W> ReceiverChannel<W>
    where W: Write
{
    pub fn new(sender: String, receiver: String, writer: Rc<RefCell<W>>) -> ReceiverChannel<W> {
        ReceiverChannel {
            sender: sender,
            receiver: receiver,
            writer: writer,
        }
    }

    pub fn launch_app(&self, app_id: String) {
        let message = MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                             self.sender.clone(),
                                             self.receiver.clone(),
                                             Some(AppLaunchRequest {
                                                 typ: MESSAGE_TYPE_LAUNCH.to_owned(),
                                                 request_id: 1,
                                                 app_id: app_id,
                                             }));
        MessageManager::send(&mut *self.writer.borrow_mut(), message);
    }

    pub fn stop_current_app(&self) {
        let message = MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                             self.sender.clone(),
                                             self.receiver.clone(),
                                             Some(AppStopRequest {
                                                 typ: MESSAGE_TYPE_STOP.to_owned(),
                                                 request_id: 1,
                                                 session_id: "FAKE".to_owned(),
                                             }));
        MessageManager::send(&mut *self.writer.borrow_mut(), message);
    }

    pub fn get_status(&self) {
        let message = MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                             self.sender.clone(),
                                             self.receiver.clone(),
                                             Some(GetStatusRequest {
                                                 typ: MESSAGE_TYPE_GET_STATUS.to_owned(),
                                                 request_id: 1,
                                             }));
        MessageManager::send(&mut *self.writer.borrow_mut(), message);
    }
}
