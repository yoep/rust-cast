use std::borrow::Cow;
use std::cell::RefCell;
use std::convert::Into;
use std::io::Write;
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;

use serde_json;

use cast::proxies;
use errors::Error;
use message_manager::{CastMessage, CastMessagePayload, MessageManager};

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.receiver";

const MESSAGE_TYPE_LAUNCH: &'static str = "LAUNCH";
const MESSAGE_TYPE_STOP: &'static str = "STOP";
const MESSAGE_TYPE_GET_STATUS: &'static str = "GET_STATUS";
const MESSAGE_TYPE_SET_VOLUME: &'static str = "SET_VOLUME";

const MESSAGE_TYPE_RECEIVER_STATUS: &'static str = "RECEIVER_STATUS";
const MESSAGE_TYPE_LAUNCH_ERROR: &'static str = "LAUNCH_ERROR";

const APP_DEFAULT_MEDIA_RECEIVER_ID: &'static str = "CC1AD845";
const APP_BACKDROP_ID: &'static str = "E8C28D3C";
const APP_YOUTUBE_ID: &'static str = "233637DE";

/// Structure that describes possible cast device volume options.
#[derive(Debug)]
pub struct Volume {
    /// Volume level.
    pub level: Option<f32>,
    /// Mute/unmute state.
    pub muted: Option<bool>,
}

/// This `Into<Volume>` implementation is useful when only volume level is needed.
impl Into<Volume> for f32 {
    fn into(self) -> Volume {
        Volume {
            level: Some(self),
            muted: None,
        }
    }
}

/// This `Into<Volume>` implementation is useful when only mute/unmute state is needed.
impl Into<Volume> for bool {
    fn into(self) -> Volume {
        Volume {
            level: None,
            muted: Some(self),
        }
    }
}

/// This `Into<Volume>` implementation is useful when both volume level and mute/unmute state are
/// needed.
impl Into<Volume> for (f32, bool) {
    fn into(self) -> Volume {
        Volume {
            level: Some(self.0),
            muted: Some(self.1),
        }
    }
}

/// Structure that describes currently run Cast Device application.
#[derive(Debug)]
pub struct Application {
    /// The identifier of the Cast application. Not for display.
    pub app_id: String,
    /// Session id of the currently active application.
    pub session_id: String,
    /// Name of the `pipe` to talk to the application.
    pub transport_id: String,
    /// A list of the namespaces supported by the receiver application.
    pub namespaces: Vec<String>,
    /// The human-readable name of the Cast application, for example, "YouTube".
    pub display_name: String,
    /// Descriptive text for the current application content, for example “My vacations”.
    pub status_text: String,
}

/// Describes the current status of the receiver cast device.
#[derive(Debug)]
pub struct Status {
    /// Contains the list of applications that are currently run.
    pub applications: Vec<Application>,
    /// Determines whether the Cast device is the active input or not.
    pub is_active_input: bool,
    /// Determines whether the Cast device is in stand by mode.
    pub is_stand_by: bool,
    /// Volume parameters of the currently active cast device.
    pub volume: Volume,
}

/// Represents all currently supported incoming messages that receiver channel can handle.
#[derive(Debug)]
pub enum ReceiverResponse {
    /// Status of the currently active receiver.
    Status(Status),
    /// Error indicating that receiver failed to launch application.
    LaunchError,
    /// Used every time when channel can't parse the message. Associated data contains `type` string
    /// field and raw JSON data returned from cast device.
    NotImplemented(String, serde_json::Value),
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
        let payload = try!(serde_json::to_string(
            &proxies::receiver::AppLaunchRequest {
                typ: MESSAGE_TYPE_LAUNCH.to_owned(),
                request_id: 10000,
                app_id: app.to_string(),
            }));

        MessageManager::send(&mut *self.writer.borrow_mut(), CastMessage {
            namespace: CHANNEL_NAMESPACE.to_owned(),
            source: self.sender.to_string(),
            destination: self.receiver.to_string(),
            payload: CastMessagePayload::String(payload),
        })
    }

    pub fn stop_app<S>(&self, session_id: S) -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = try!(serde_json::to_string(
            &proxies::receiver::AppStopRequest {
                typ: MESSAGE_TYPE_STOP.to_owned(),
                request_id: 20000,
                session_id: session_id.into(),
            }));

        MessageManager::send(&mut *self.writer.borrow_mut(), CastMessage {
            namespace: CHANNEL_NAMESPACE.to_owned(),
            source: self.sender.to_string(),
            destination: self.receiver.to_string(),
            payload: CastMessagePayload::String(payload),
        })
    }

    pub fn get_status(&self) -> Result<(), Error> {
        let payload = try!(serde_json::to_string(
            &proxies::receiver::GetStatusRequest {
                typ: MESSAGE_TYPE_GET_STATUS.to_owned(),
                request_id: 30000,
            }));

        MessageManager::send(&mut *self.writer.borrow_mut(), CastMessage {
            namespace: CHANNEL_NAMESPACE.to_owned(),
            source: self.sender.to_string(),
            destination: self.receiver.to_string(),
            payload: CastMessagePayload::String(payload),
        })
    }

    /// Sets volume for the active cast device.
    ///
    /// # Arguments
    ///
    /// * `volume` - anything that can be converted to a valid `Volume` structure. It's possible to
    ///              set volume level, mute/unmute state or both altogether.
    ///
    /// # Errors
    ///
    /// Usually method can fail only if network connection with cast device is lost for some reason.
    pub fn set_volume<T>(&self, volume: T) -> Result<(), Error> where T: Into<Volume> {
        let volume = volume.into();

        let payload = try!(serde_json::to_string(
            &proxies::receiver::SetVolumeRequest {
                typ: MESSAGE_TYPE_SET_VOLUME.to_owned(),
                request_id: 40000,
                volume: proxies::receiver::Volume {
                    level: volume.level,
                    muted: volume.muted,
                },
            }));

        MessageManager::send(&mut *self.writer.borrow_mut(), CastMessage {
            namespace: CHANNEL_NAMESPACE.to_owned(),
            source: self.sender.to_string(),
            destination: self.receiver.to_string(),
            payload: CastMessagePayload::String(payload),
        })
    }

    pub fn can_handle(&self, message: &CastMessage) -> bool {
        message.namespace == CHANNEL_NAMESPACE
    }

    pub fn parse(&self, message: &CastMessage) -> Result<ReceiverResponse, Error> {
        let reply = match message.payload {
            CastMessagePayload::String(ref payload) => try!(
                serde_json::from_str::<serde_json::Value>(payload)),
            _ => return Err(Error::Internal("Binary payload is not supported!".to_owned())),
        };

        let message_type = reply.as_object()
            .and_then(|object| object.get("type"))
            .and_then(|property| property.as_string())
            .unwrap_or("")
            .to_owned();

        let response = match message_type.as_ref() {
            MESSAGE_TYPE_RECEIVER_STATUS => {
                let status_reply: proxies::receiver::StatusReply = try!(
                    serde_json::value::from_value(reply));

                let status = Status {
                    applications: status_reply.status.applications.iter().map(|ref app| {
                        Application {
                            app_id: app.app_id.clone(),
                            session_id: app.session_id.clone(),
                            transport_id: app.transport_id.clone(),
                            namespaces: app.namespaces.iter().map(|ref ns| ns.name.clone())
                                .collect::<Vec<String>>(),
                            display_name: app.display_name.clone(),
                            status_text: app.status_text.clone(),
                        }
                    }).collect::<Vec<Application>>(),
                    is_active_input: status_reply.status.is_active_input,
                    is_stand_by: status_reply.status.is_stand_by,
                    volume: Volume {
                        level: status_reply.status.volume.level,
                        muted: status_reply.status.volume.muted,
                    },
                };

                ReceiverResponse::Status(status)
            },
            MESSAGE_TYPE_LAUNCH_ERROR => ReceiverResponse::LaunchError,
            _ => ReceiverResponse::NotImplemented(message_type.to_owned(), reply),
        };

        Ok(response)
    }
}
