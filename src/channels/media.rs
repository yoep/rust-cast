use std::borrow::Cow;
use std::cell::RefCell;
use std::str::FromStr;
use std::string::ToString;
use std::io::Write;
use std::rc::Rc;

use serde_json;

use cast::proxies;
use errors::Error;
use message_manager::{CastMessage, CastMessagePayload, MessageManager};

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.media";

const MESSAGE_TYPE_GET_STATUS: &'static str = "GET_STATUS";
const MESSAGE_TYPE_LOAD: &'static str = "LOAD";
const MESSAGE_TYPE_PLAY: &'static str = "PLAY";
const MESSAGE_TYPE_PAUSE: &'static str = "PAUSE";
const MESSAGE_TYPE_STOP: &'static str = "STOP";
const MESSAGE_TYPE_SEEK: &'static str = "SEEK";
const MESSAGE_TYPE_MEDIA_STATUS: &'static str = "MEDIA_STATUS";
const MESSAGE_TYPE_LOAD_CANCELLED: &'static str = "LOAD_CANCELLED";

/// Describes the way cast device should stream content.
#[derive(Debug)]
pub enum StreamType {
    /// This variant allows cast device to automatically choose whatever way it's most comfortable
    /// with.
    None,
    /// Cast device should buffer some portion of the content and only then start streaming.
    Buffered,
    /// Cast device should display content as soon as it gets any portion of it.
    Live,
}

impl FromStr for StreamType {
    type Err = Error;

    fn from_str(s: &str) -> Result<StreamType, Error> {
        match s {
            "BUFFERED"  => Ok(StreamType::Buffered),
            "LIVE" => Ok(StreamType::Live),
            _ => Ok(StreamType::None),
        }
    }
}

impl ToString for StreamType {
    fn to_string(&self) -> String {
        let stream_type = match *self {
            StreamType::None => "NONE",
            StreamType::Buffered => "BUFFERED",
            StreamType::Live => "LIVE",
        };

        stream_type.to_owned()
    }
}

/// Describes possible player states.
#[derive(Debug)]
pub enum PlayerState {
    /// Player has not been loaded yet.
    Idle,
    /// Player is actively playing content.
    Playing,
    /// Player is in PLAY mode but not actively playing content (currentTime is not changing).
    Buffering,
    /// Player is paused.
    Paused,
}

impl FromStr for PlayerState {
    type Err = Error;

    fn from_str(s: &str) -> Result<PlayerState, Error> {
        match s {
            "IDLE"  => Ok(PlayerState::Idle),
            "PLAYING" => Ok(PlayerState::Playing),
            "BUFFERING" => Ok(PlayerState::Buffering),
            "PAUSED" => Ok(PlayerState::Paused),
            _ => Err(Error::Internal(format!("Unknown player state {}", s))),
        }
    }
}

/// Describes possible player idle reasons.
#[derive(Debug)]
pub enum IdleReason {
    /// A sender requested to stop playback using the STOP command.
    Cancelled,
    /// A sender requested playing a different media using the LOAD command.
    Interrupted,
    /// The media playback completed.
    Finished,
    /// The media was interrupted due to an error; For example, if the player could not download the
    /// media due to network issues.
    Error,
}

impl FromStr for IdleReason {
    type Err = Error;

    fn from_str(s: &str) -> Result<IdleReason, Error> {
        match s {
            "CANCELLED"  => Ok(IdleReason::Cancelled),
            "INTERRUPTED" => Ok(IdleReason::Interrupted),
            "FINISHED" => Ok(IdleReason::Finished),
            "ERROR" => Ok(IdleReason::Error),
            _ => Err(Error::Internal(format!("Unknown idle reason {}", s))),
        }
    }
}

/// This data structure describes a media stream.
#[derive(Debug)]
pub struct Media {
    /// Service-specific identifier of the content currently loaded by the media player. This is a
    /// free form string and is specific to the application. In most cases, this will be the URL to
    /// the media, but the sender can choose to pass a string that the receiver can interpret
    /// properly. Max length: 1k.
    pub content_id: String,
    /// Describes the type of media artifact.
    pub stream_type: StreamType,
    /// MIME content type of the media being played.
    pub content_type: String,
    /// Duration of the currently playing stream in seconds.
    pub duration: Option<f32>,
}

/// Describes the current status of the media artifact with respect to the session.
#[derive(Debug)]
pub struct Status {
    pub media_session_id: i32,
    pub media: Option<Media>,
    pub playback_rate: f32,
    pub player_state: PlayerState,
    pub idle_reason: Option<IdleReason>,
    pub current_time: f32,
    pub supported_media_commands: u8,
}

/// Represents all currently supported incoming messages that media channel can handle.
#[derive(Debug)]
pub enum MediaResponse {
    /// Statuses of the currently active media.
    Status(Vec<Status>),
    /// Information about cancelled media.
    LoadCancelled,
    /// Used every time when channel can't parse the message. Associated data contains `type` string
    /// field and raw JSON data returned from cast device.
    NotImplemented(String, serde_json::Value),
}

pub struct MediaChannel<'a, W> where W: Write {
    sender: Cow<'a, str>,
    writer: Rc<RefCell<W>>,
}

impl<'a, W> MediaChannel<'a, W> where W: Write {
    pub fn new<S>(sender: S, writer: Rc<RefCell<W>>)
        -> MediaChannel<'a, W> where S: Into<Cow<'a, str>> {
        MediaChannel {
            sender: sender.into(),
            writer: writer,
        }
    }

    pub fn get_status<S>(&self, destination: S) -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = try!(serde_json::to_string(
            &proxies::media::GetStatusRequest {
                typ: MESSAGE_TYPE_GET_STATUS.to_owned(),
                request_id: 1000,
                media_session_id: None,
            }));

        MessageManager::send(&mut *self.writer.borrow_mut(), CastMessage {
            namespace: CHANNEL_NAMESPACE.to_owned(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        })
    }

    pub fn load<S>(&self, destination: S, session_id: S, content_id: S, content_type: S,
                   stream_type: StreamType) -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = try!(serde_json::to_string(
            &proxies::media::MediaRequest {
                request_id: 2000,
                session_id: session_id.into().to_string(),
                typ: MESSAGE_TYPE_LOAD.to_owned(),

                media: proxies::media::Media {
                    content_id: content_id.into().to_string(),
                    stream_type: stream_type.to_string(),
                    content_type: content_type.into().to_string(),
                    duration: None,
                },

                current_time: 0_f64,
                autoplay: true,
                custom_data: proxies::media::CustomData::new(),
            }));

        MessageManager::send(&mut *self.writer.borrow_mut(), CastMessage {
            namespace: CHANNEL_NAMESPACE.to_owned(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        })
    }

    pub fn pause<S>(&self, destination: S, media_session_id: S)
        -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = try!(serde_json::to_string(
            &proxies::media::PauseRequest {
                request_id: 3000,
                media_session_id: media_session_id.into().to_string(),
                typ: MESSAGE_TYPE_PAUSE.to_owned(),
                custom_data: proxies::media::CustomData::new(),
            }));

        MessageManager::send(&mut *self.writer.borrow_mut(), CastMessage {
            namespace: CHANNEL_NAMESPACE.to_owned(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        })
    }

    pub fn can_handle(&self, message: &CastMessage) -> bool {
        message.namespace == CHANNEL_NAMESPACE
    }

    pub fn parse(&self, message: &CastMessage) -> Result<MediaResponse, Error> {
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
            MESSAGE_TYPE_MEDIA_STATUS => {
                let status_reply: proxies::media::StatusReply = try!(
                    serde_json::value::from_value(reply));

                let statuses = status_reply.status.iter().map(|ref x| {
                    Status {
                        media_session_id: x.media_session_id,
                        media: x.media.as_ref().map(|ref m| Media {
                            content_id: m.content_id.to_owned(),
                            stream_type: StreamType::from_str(m.stream_type.as_ref()).unwrap(),
                            content_type: m.content_type.to_owned(),
                            duration: m.duration,
                        }),
                        playback_rate: x.playback_rate,
                        player_state: PlayerState::from_str(x.player_state.as_ref()).unwrap(),
                        idle_reason: x.idle_reason.as_ref().map(
                            |ref reason| IdleReason::from_str(reason).unwrap()),
                        current_time: x.current_time,
                        supported_media_commands: x.supported_media_commands,
                    }
                });

                MediaResponse::Status(statuses.collect::<Vec<Status>>())
            },
            MESSAGE_TYPE_LOAD_CANCELLED => MediaResponse::LoadCancelled,
            _ => MediaResponse::NotImplemented(message_type.to_owned(), reply),
        };

        Ok(response)
    }
}
