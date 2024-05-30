use std::{borrow::Cow, convert::Into, fmt, io::Write, str::FromStr, string::ToString};

use serde::Serialize;

use crate::{
    cast::proxies,
    errors::Error,
    message_manager::{CastMessage, CastMessagePayload, MessageManager},
    Lrc,
};

const CHANNEL_NAMESPACE: &str = "urn:x-cast:com.google.cast.receiver";

const MESSAGE_TYPE_LAUNCH: &str = "LAUNCH";
const MESSAGE_TYPE_STOP: &str = "STOP";
const MESSAGE_TYPE_GET_STATUS: &str = "GET_STATUS";
const MESSAGE_TYPE_SET_VOLUME: &str = "SET_VOLUME";

const MESSAGE_TYPE_RECEIVER_STATUS: &str = "RECEIVER_STATUS";
const MESSAGE_TYPE_LAUNCH_ERROR: &str = "LAUNCH_ERROR";
const MESSAGE_TYPE_INVALID_REQUEST: &str = "INVALID_REQUEST";

const APP_DEFAULT_MEDIA_RECEIVER_ID: &str = "CC1AD845";
const APP_BACKDROP_ID: &str = "E8C28D3C";
const APP_YOUTUBE_ID: &str = "233637DE";

/// Structure that describes possible cast device volume options.
#[derive(Copy, Clone, Debug)]
pub struct Volume {
    /// Volume level.
    pub level: Option<f32>,
    /// Mute/unmute state.
    pub muted: Option<bool>,
}

/// This `From<f32>` implementation is useful when only volume level is needed.
impl From<f32> for Volume {
    fn from(level: f32) -> Self {
        Self {
            level: Some(level),
            muted: None,
        }
    }
}

/// This `From<bool>` implementation is useful when only mute/unmute state is needed.
impl From<bool> for Volume {
    fn from(muted: bool) -> Self {
        Self {
            level: None,
            muted: Some(muted),
        }
    }
}

/// This `From<(f32, bool)>` implementation is useful when both volume level and mute/unmute state are
/// needed.
impl From<(f32, bool)> for Volume {
    fn from((level, muted): (f32, bool)) -> Self {
        Self {
            level: Some(level),
            muted: Some(muted),
        }
    }
}

/// Structure that describes currently run Cast Device application.
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug)]
pub struct Status {
    /// Unique id of the request that requested the status.
    pub request_id: u32,
    /// Contains the list of applications that are currently run.
    pub applications: Vec<Application>,
    /// Determines whether the Cast device is the active input or not.
    pub is_active_input: bool,
    /// Determines whether the Cast device is in stand by mode.
    pub is_stand_by: bool,
    /// Volume parameters of the currently active cast device.
    pub volume: Volume,
}

/// Describes the application launch error.
#[derive(Clone, Debug)]
pub struct LaunchError {
    /// Unique id of the request that tried to launch application.
    pub request_id: u32,
    /// Description of the launch error reason if available.
    pub reason: Option<String>,
}

/// Describes the invalid request error.
#[derive(Clone, Debug)]
pub struct InvalidRequest {
    /// Unique id of the invalid request.
    pub request_id: u32,
    /// Description of the invalid request reason if available.
    pub reason: Option<String>,
}

/// Represents all currently supported incoming messages that receiver channel can handle.
#[derive(Clone, Debug)]
pub enum ReceiverResponse {
    /// Status of the currently active receiver.
    Status(Status),
    /// Error indicating that receiver failed to launch application.
    LaunchError(LaunchError),
    /// Error indicating that request is not valid.
    InvalidRequest(InvalidRequest),
    /// Used every time when channel can't parse the message. Associated data contains `type` string
    /// field and raw JSON data returned from cast device.
    NotImplemented(String, serde_json::Value),
}

#[derive(Clone, Debug, PartialEq)]
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
            custom => CastDeviceApp::Custom(custom.to_string()),
        };

        Ok(app)
    }
}

impl fmt::Display for CastDeviceApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match *self {
            CastDeviceApp::DefaultMediaReceiver => APP_DEFAULT_MEDIA_RECEIVER_ID.to_string(),
            CastDeviceApp::Backdrop => APP_BACKDROP_ID.to_string(),
            CastDeviceApp::YouTube => APP_YOUTUBE_ID.to_string(),
            CastDeviceApp::Custom(ref app_id) => app_id.to_string(),
        };
        write!(f, "{}", str)
    }
}

pub struct ReceiverChannel<'a, W>
where
    W: Write,
{
    sender: Cow<'a, str>,
    receiver: Cow<'a, str>,
    message_manager: Lrc<MessageManager<W>>,
}

impl<'a, W> ReceiverChannel<'a, W>
where
    W: Write,
{
    pub fn new<S>(
        sender: S,
        receiver: S,
        message_manager: Lrc<MessageManager<W>>,
    ) -> ReceiverChannel<'a, W>
    where
        S: Into<Cow<'a, str>>,
    {
        ReceiverChannel {
            sender: sender.into(),
            receiver: receiver.into(),
            message_manager,
        }
    }

    /// Launches the specified receiver's application.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::str::FromStr;
    /// use rust_cast::{CastDevice, channels::receiver::CastDeviceApp};
    ///
    /// # let cast_device = CastDevice::connect_without_host_verification("host", 1234).unwrap();
    /// cast_device.receiver.launch_app(&CastDeviceApp::from_str("youtube").unwrap());
    /// ```
    ///
    /// # Arguments
    ///
    /// * `app` - `CastDeviceApp` instance reference to run.
    pub fn launch_app(&self, app: &CastDeviceApp) -> Result<Application, Error> {
        let request_id = self.message_manager.generate_request_id().get();

        let payload = serde_json::to_string(&proxies::receiver::AppLaunchRequest {
            typ: MESSAGE_TYPE_LAUNCH.to_string(),
            request_id,
            app_id: app.to_string(),
        })?;

        self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: self.receiver.to_string(),
            payload: CastMessagePayload::String(payload),
        })?;

        // Once application is run cast receiver device should emit status update event, or launch
        // error event if something went wrong.
        self.message_manager.subscribe_find(|message| {
            if !self.can_handle(message) {
                return Ok(None);
            }

            match self.parse(message)? {
                ReceiverResponse::Status(mut status) => {
                    if status.request_id == request_id {
                        return Ok(Some(status.applications.remove(0)));
                    }
                }
                ReceiverResponse::LaunchError(error) => {
                    if error.request_id == request_id {
                        return Err(Error::Internal(format!(
                            "Could not run application ({}).",
                            error.reason.unwrap_or_else(|| "Unknown".to_string())
                        )));
                    }
                }
                _ => {}
            }

            Ok(None)
        })
    }

    /// Broadcasts a message over a cast device's message bus.
    ///
    /// Receiver can observe messages using `context.addCustomMessageListener` with custom namespace.
    ///
    /// ```javascript, no_run
    /// context.addCustomMessageListener('urn:x-cast:com.example.castdata', function(customEvent) {
    ///   // do something with message
    /// });
    /// ```
    ///
    /// Namespace should start with `urn:x-cast:`
    ///
    /// # Arguments
    ///
    /// * `namespace` - Message namespace that should start with `urn:x-cast:`.
    /// * `message` - Message instance to send.
    pub fn broadcast_message<M: Serialize>(
        &self,
        namespace: &str,
        message: &M,
    ) -> Result<(), Error> {
        if !namespace.starts_with("urn:x-cast:") {
            return Err(Error::Namespace(format!(
                "'{}' should start with 'urn:x-cast:' prefix",
                namespace
            )));
        }
        let payload = serde_json::to_string(message)?;
        self.message_manager.send(CastMessage {
            namespace: namespace.to_string(),
            source: self.sender.to_string(),
            destination: "*".into(),
            payload: CastMessagePayload::String(payload),
        })?;

        Ok(())
    }

    /// Stops currently active app using corresponding `session_id`.
    ///
    /// # Arguments
    /// * `session_id` - identifier of the active application session from `Application` instance.
    pub fn stop_app<S>(&self, session_id: S) -> Result<(), Error>
    where
        S: Into<Cow<'a, str>>,
    {
        let request_id = self.message_manager.generate_request_id().get();

        let payload = serde_json::to_string(&proxies::receiver::AppStopRequest {
            typ: MESSAGE_TYPE_STOP.to_string(),
            request_id,
            session_id: session_id.into(),
        })?;

        self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: self.receiver.to_string(),
            payload: CastMessagePayload::String(payload),
        })?;

        // Once application is stopped cast receiver device should emit status update event, or
        // invalid request event if provided session id is not valid.
        self.message_manager.subscribe_find(|message| {
            if !self.can_handle(message) {
                return Ok(None);
            }

            match self.parse(message)? {
                ReceiverResponse::Status(status) => {
                    if status.request_id == request_id {
                        return Ok(Some(()));
                    }
                }
                ReceiverResponse::InvalidRequest(error) => {
                    if error.request_id == request_id {
                        return Err(Error::Internal(format!(
                            "Invalid request ({}).",
                            error.reason.unwrap_or_else(|| "Unknown".to_string())
                        )));
                    }
                }
                _ => {}
            }

            Ok(None)
        })
    }

    /// Retrieves status of the cast device receiver.
    ///
    /// # Return value
    ///
    /// Returned `Result` should consist of either `Status` instance or an `Error`.
    pub fn get_status(&self) -> Result<Status, Error> {
        let request_id = self.message_manager.generate_request_id().get();

        let payload = serde_json::to_string(&proxies::receiver::GetStatusRequest {
            typ: MESSAGE_TYPE_GET_STATUS.to_string(),
            request_id,
        })?;

        self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: self.receiver.to_string(),
            payload: CastMessagePayload::String(payload),
        })?;

        self.message_manager.subscribe_find(|message| {
            if !self.can_handle(message) {
                return Ok(None);
            }

            let message = self.parse(message)?;
            if let ReceiverResponse::Status(status) = message {
                if status.request_id == request_id {
                    return Ok(Some(status));
                }
            }

            Ok(None)
        })
    }

    /// Sets volume for the active cast device.
    ///
    /// # Arguments
    ///
    /// * `volume` - anything that can be converted to a valid `Volume` structure. It's possible to
    ///              set volume level, mute/unmute state or both altogether.
    ///
    /// # Return value
    ///
    /// Actual `Volume` instance returned by receiver.
    ///
    /// # Errors
    ///
    /// Usually method can fail only if network connection with cast device is lost for some reason.
    pub fn set_volume<T>(&self, volume: T) -> Result<Volume, Error>
    where
        T: Into<Volume>,
    {
        let request_id = self.message_manager.generate_request_id().get();
        let volume = volume.into();

        let payload = serde_json::to_string(&proxies::receiver::SetVolumeRequest {
            typ: MESSAGE_TYPE_SET_VOLUME.to_string(),
            request_id,
            volume: proxies::receiver::Volume {
                level: volume.level,
                muted: volume.muted,
            },
        })?;

        self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: self.receiver.to_string(),
            payload: CastMessagePayload::String(payload),
        })?;

        self.message_manager.subscribe_find(|message| {
            if !self.can_handle(message) {
                return Ok(None);
            }

            let message = self.parse(message)?;
            if let ReceiverResponse::Status(status) = message {
                if status.request_id == request_id {
                    return Ok(Some(status.volume));
                }
            }

            Ok(None)
        })
    }

    pub fn can_handle(&self, message: &CastMessage) -> bool {
        message.namespace == CHANNEL_NAMESPACE
    }

    pub fn parse(&self, message: &CastMessage) -> Result<ReceiverResponse, Error> {
        let reply = match message.payload {
            CastMessagePayload::String(ref payload) => {
                serde_json::from_str::<serde_json::Value>(payload)?
            }
            _ => {
                return Err(Error::Internal(
                    "Binary payload is not supported!".to_string(),
                ))
            }
        };

        let message_type = reply
            .as_object()
            .and_then(|object| object.get("type"))
            .and_then(|property| property.as_str())
            .unwrap_or("")
            .to_string();

        let response = match message_type.as_ref() {
            MESSAGE_TYPE_RECEIVER_STATUS => {
                let status_reply: proxies::receiver::StatusReply =
                    serde_json::value::from_value(reply)?;

                let status = Status {
                    request_id: status_reply.request_id,
                    applications: status_reply
                        .status
                        .applications
                        .iter()
                        .map(|app| Application {
                            app_id: app.app_id.clone(),
                            session_id: app.session_id.clone(),
                            transport_id: app.transport_id.clone(),
                            namespaces: app
                                .namespaces
                                .iter()
                                .map(|ns| ns.name.clone())
                                .collect::<Vec<String>>(),
                            display_name: app.display_name.clone(),
                            status_text: app.status_text.clone(),
                        })
                        .collect::<Vec<Application>>(),
                    is_active_input: status_reply.status.is_active_input,
                    is_stand_by: status_reply.status.is_stand_by,
                    volume: Volume {
                        level: status_reply.status.volume.level,
                        muted: status_reply.status.volume.muted,
                    },
                };

                ReceiverResponse::Status(status)
            }
            MESSAGE_TYPE_LAUNCH_ERROR => {
                let reply: proxies::receiver::LaunchErrorReply =
                    serde_json::value::from_value(reply)?;

                ReceiverResponse::LaunchError(LaunchError {
                    request_id: reply.request_id,
                    reason: reply.reason,
                })
            }
            MESSAGE_TYPE_INVALID_REQUEST => {
                let reply: proxies::receiver::InvalidRequestReply =
                    serde_json::value::from_value(reply)?;

                ReceiverResponse::InvalidRequest(InvalidRequest {
                    request_id: reply.request_id,
                    reason: reply.reason,
                })
            }
            _ => ReceiverResponse::NotImplemented(message_type.to_string(), reply),
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use protobuf::EnumOrUnknown;

    use crate::cast::cast_channel;
    use crate::cast::cast_channel::cast_message::{PayloadType, ProtocolVersion};
    use crate::tests::{init_logger, MockTcpStream};
    use crate::{DEFAULT_RECEIVER_ID, DEFAULT_SENDER_ID};

    use super::*;

    #[test]
    fn test_launch_app() {
        init_logger();
        let app_id = "MyAppId";
        let session_id = "MySessionId";
        let transport_id = "MyTransportId";
        let display_name = "MyDisplayName";
        let status_text = "Idle";
        let payload = format!(
            r#"{{
            "requestId":1,
            "type":"{type}",
            "status":{{
                "applications": [
                   {{
                    "appId": "{app_id}",
                    "sessionId": "{session_id}",
                    "transportId": "{transport_id}",
                    "namespaces": [],
                    "displayName": "{display_name}",
                    "statusText": "{status_text}"
                   }}
                ],
                "isActiveInput": true,
                "isStandBy": true,
                "volume": {{
                    "level": 1.0,
                    "muted": false                    
                }}
            }}
            }}"#,
            type = MESSAGE_TYPE_RECEIVER_STATUS,
            app_id = app_id,
            session_id = session_id,
            transport_id = transport_id,
            display_name = display_name,
            status_text = status_text
        );
        let mut stream = MockTcpStream::new();
        stream.add_message(cast_channel::CastMessage {
            protocol_version: Some(EnumOrUnknown::new(ProtocolVersion::CASTV2_1_2)),
            source_id: Some(DEFAULT_RECEIVER_ID.to_string()),
            destination_id: Some(DEFAULT_SENDER_ID.to_string()),
            namespace: Some(CHANNEL_NAMESPACE.to_string()),
            payload_type: Some(EnumOrUnknown::new(PayloadType::STRING)),
            payload_utf8: Some(payload),
            payload_binary: None,
            continued: None,
            remaining_length: None,
            special_fields: Default::default(),
        });
        let (reader, writer) = stream.split();
        let message_manager = Lrc::new(MessageManager::new(reader, writer));
        let channel = ReceiverChannel::new(DEFAULT_SENDER_ID, DEFAULT_RECEIVER_ID, message_manager);

        // verify if we got the expected response from the mock stream
        let expected_application = Application {
            app_id: app_id.to_string(),
            session_id: session_id.to_string(),
            transport_id: transport_id.to_string(),
            namespaces: vec![],
            display_name: display_name.to_string(),
            status_text: status_text.to_string(),
        };
        let result = channel
            .launch_app(&CastDeviceApp::DefaultMediaReceiver)
            .unwrap();
        assert_eq!(expected_application, result);

        // verify if the expected message has been sent to the mock stream
        let expected_message = r#"{"requestId":1,"type":"LAUNCH","appId":"CC1AD845"}"#;
        let message = stream
            .received_message(0)
            .expect("expected a message to have been received");
        let message = CastMessage::from(message.message());
        assert_eq!(
            CastMessagePayload::String(expected_message.to_string()),
            message.payload
        );
    }

    #[test]
    fn test_broadcast_message() {
        init_logger();
        let namespace = "urn:x-cast:com.google.cast.media";
        let destination = "*";
        let payload = r#"{"requestId":1,"type":"GET_STATUS"}"#;
        let broadcast_message = cast_channel::CastMessage {
            protocol_version: Some(EnumOrUnknown::new(ProtocolVersion::CASTV2_1_0)),
            source_id: Some(DEFAULT_SENDER_ID.to_string()),
            destination_id: Some(destination.to_string()),
            namespace: Some(namespace.to_string()),
            payload_type: Some(EnumOrUnknown::new(PayloadType::STRING)),
            payload_utf8: Some(payload.to_string()),
            payload_binary: None,
            continued: None,
            remaining_length: None,
            special_fields: Default::default(),
        };
        let stream = MockTcpStream::new();
        let (reader, writer) = stream.split();
        let message_manager = Lrc::new(MessageManager::new(reader, writer));
        let channel = ReceiverChannel::new(DEFAULT_SENDER_ID, DEFAULT_RECEIVER_ID, message_manager);

        channel
            .broadcast_message(namespace, &broadcast_message.payload_utf8())
            .expect("expected the message to have been broadcast");

        let message = stream
            .received_message(0)
            .expect("expected a message to have been received");
        let message = message.message();
        assert_eq!(DEFAULT_SENDER_ID, message.source_id());
        assert_eq!(destination, message.destination_id());
        assert_eq!(namespace, message.namespace());
    }
}
