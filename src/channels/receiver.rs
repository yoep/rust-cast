use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;
use serde_json::Value;
use serde_json::value::from_value;

use cast::cast_channel;
use errors::Error;
use message_manager::MessageManager;

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.receiver";

const MESSAGE_TYPE_LAUNCH: &'static str = "LAUNCH";
const MESSAGE_TYPE_STOP: &'static str = "STOP";
const MESSAGE_TYPE_GET_STATUS: &'static str = "GET_STATUS";

const MESSAGE_TYPE_RECEIVER_STATUS: &'static str = "RECEIVER_STATUS";
const MESSAGE_TYPE_LAUNCH_ERROR: &'static str = "LAUNCH_ERROR";

const APP_DEFAULT_MEDIA_RECEIVER_ID: &'static str = "CC1AD845";
const APP_BACKDROP_ID: &'static str = "E8C28D3C";
const APP_YOUTUBE_ID: &'static str = "233637DE";

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
struct AppStopRequest<'a> {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="type")]
    pub typ: String,

    #[serde(rename="sessionId")]
    pub session_id: Cow<'a, str>,
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

    #[serde(rename="transportId", default)]
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
pub enum ReceiverResponse {
    Status(StatusReply),
    LaunchError(LaunchErrorReply),
    NotImplemented(String, Value),
}

#[derive(Debug, PartialEq)]
pub enum CastDeviceApp {
    DefaultMediaReceiver,
    Backdrop,
    YouTube,
    Custom(String),
}

impl FromStr for CastDeviceApp {
    type Err = ();

    fn from_str(s: &str) -> Result<CastDeviceApp, ()> {
        let app = match s {
            APP_DEFAULT_MEDIA_RECEIVER_ID | "default" => CastDeviceApp::DefaultMediaReceiver,
            APP_BACKDROP_ID | "backdrop" => CastDeviceApp::Backdrop,
            APP_YOUTUBE_ID | "youtube" => CastDeviceApp::YouTube,
            custom @ _ => CastDeviceApp::Custom(custom.to_owned())
        };

        Ok(app)
    }
}

impl ToString for CastDeviceApp {
    fn to_string(&self) -> String {
        match *self {
            CastDeviceApp::DefaultMediaReceiver => APP_DEFAULT_MEDIA_RECEIVER_ID.to_owned(),
            CastDeviceApp::Backdrop => APP_BACKDROP_ID.to_owned(),
            CastDeviceApp::YouTube => APP_YOUTUBE_ID.to_owned(),
            CastDeviceApp::Custom(ref app_id) => app_id.to_owned(),
        }
    }
}

pub struct ReceiverChannel<'a, W> where W: Write {
    sender: Cow<'a, str>,
    receiver: Cow<'a, str>,
    writer: Rc<RefCell<W>>,
}

impl<'a, W> ReceiverChannel<'a, W> where W: Write {
    pub fn new<S>(sender: S, receiver: S, writer: Rc<RefCell<W>>)
        -> ReceiverChannel<'a, W> where S: Into<Cow<'a, str>> {
        ReceiverChannel {
            sender: sender.into(),
            receiver: receiver.into(),
            writer: writer,
        }
    }

    pub fn launch_app(&self, app: CastDeviceApp) -> Result<(), Error> {
        let payload = AppLaunchRequest {
            typ: MESSAGE_TYPE_LAUNCH.to_owned(),
            request_id: 1,
            app_id: app.to_string(),
        };

        MessageManager::send(&mut *self.writer.borrow_mut(), CHANNEL_NAMESPACE.to_owned(),
                             self.sender.to_string(), self.receiver.to_string(), Some(payload))
    }

    pub fn stop_app<S>(&self, session_id: S) -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = AppStopRequest {
            typ: MESSAGE_TYPE_STOP.to_owned(),
            request_id: 1,
            session_id: session_id.into(),
        };

        MessageManager::send(&mut *self.writer.borrow_mut(), CHANNEL_NAMESPACE.to_owned(),
                             self.sender.to_string(), self.receiver.to_string(), Some(payload))
    }

    pub fn get_status(&self) -> Result<(), Error> {
        let payload = GetStatusRequest {
            typ: MESSAGE_TYPE_GET_STATUS.to_owned(),
            request_id: 1,
        };

        MessageManager::send(&mut *self.writer.borrow_mut(), CHANNEL_NAMESPACE.to_owned(),
                             self.sender.to_string(), self.receiver.to_string(), Some(payload))
    }

    pub fn can_handle(&self, message: &cast_channel::CastMessage) -> bool {
        message.get_namespace() == CHANNEL_NAMESPACE
    }

    pub fn parse(&self, message: &cast_channel::CastMessage) -> Result<ReceiverResponse, Error> {
        let reply: Value = try!(MessageManager::parse_payload(message));

        let message_type = reply.as_object()
            .and_then(|object| object.get("type"))
            .and_then(|property| property.as_string())
            .unwrap_or("")
            .to_owned();

        let response = match message_type.as_ref() {
            MESSAGE_TYPE_RECEIVER_STATUS => ReceiverResponse::Status(try!(from_value(reply))),
            MESSAGE_TYPE_LAUNCH_ERROR => ReceiverResponse::LaunchError(try!(from_value(reply))),
            _ => ReceiverResponse::NotImplemented(message_type.to_owned(), reply),
        };

        Ok(response)
    }
}
