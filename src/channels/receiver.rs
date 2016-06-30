use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use serde_json::Value;
use serde_json::value::from_value;

use cast::cast_channel;
use errors::Error;
use message_manager::MessageManager;

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.receiver";

const MESSAGE_TYPE_LAUNCH: &'static str = "LAUNCH";
const MESSAGE_TYPE_STOP: &'static str = "STOP";
const MESSAGE_TYPE_GET_STATUS: &'static str = "GET_STATUS";

const REPLY_TYPE_RECEIVER_STATUS: &'static str = "RECEIVER_STATUS";
const REPLY_TYPE_LAUNCH_ERROR: &'static str = "LAUNCH_ERROR";

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

#[derive(Deserialize, Debug)]
pub struct StatusReply {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="type")]
    pub typ: String,

    pub status: ReceiverStatus,
}

#[derive(Deserialize, Debug)]
pub struct ReceiverStatus {
    #[serde(default)]
    pub applications: Vec<Application>,

    #[serde(rename="isActiveInput", default)]
    pub is_active_input: bool,

    #[serde(rename="isStandBy", default)]
    pub is_stand_by: bool,

    pub volume: ReceiverVolume,
}

#[derive(Deserialize, Debug)]
pub struct Application {
    #[serde(rename="appId")]
    pub app_id: String,

    #[serde(rename="sessionId")]
    pub session_id: String,

    #[serde(rename="transportId")]
    pub transport_id: String,

    #[serde(default)]
    pub namespaces: Vec<AppNamespace>,

    #[serde(rename="displayName")]
    pub display_name: String,

    #[serde(rename="statusText")]
    pub status_text: String,
}

#[derive(Deserialize, Debug)]
pub struct AppNamespace {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct ReceiverVolume {
    pub level: f64,
    pub muted: bool,
}

#[derive(Deserialize, Debug)]
pub struct LaunchErrorReply {
    #[serde(rename="type")]
    typ: String,
}

#[derive(Debug)]
pub enum Reply {
    Status(StatusReply),
    LaunchError(LaunchErrorReply),
    Unknown,
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

    pub fn try_handle(&self, message: &cast_channel::CastMessage) -> Result<Reply, Error> {
        if message.get_namespace() != CHANNEL_NAMESPACE {
            return Err(Error::Internal("Channel does not support provided message.".to_owned()));
        }

        let reply: Value = try!(MessageManager::parse_payload(message));

        let message_type = {
            let reply_object_value = reply.as_object().unwrap();
            reply_object_value.get("type").unwrap().as_string().unwrap().to_owned()
        };

        let reply = match &message_type as &str {
            REPLY_TYPE_RECEIVER_STATUS => Reply::Status(from_value(reply).unwrap()),
            REPLY_TYPE_LAUNCH_ERROR => Reply::LaunchError(from_value(reply).unwrap()),
            _ => Reply::Unknown,
        };

        Ok(reply)
    }
}
