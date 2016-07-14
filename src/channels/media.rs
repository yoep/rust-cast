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

/// Describes the operation to perform with playback while seeking.
#[derive(Debug)]
pub enum ResumeState {
    /// Forces media to start.
    PlaybackStart,
    /// Forces media to pause.
    PlaybackPause,
}

impl FromStr for ResumeState {
    type Err = Error;

    fn from_str(s: &str) -> Result<ResumeState, Error> {
        match s {
            "PLAYBACK_START" | "start"  => Ok(ResumeState::PlaybackStart),
            "PLAYBACK_PAUSE" | "pause" => Ok(ResumeState::PlaybackPause),
            _ => Err(Error::Internal(format!("Unknown resume state {}", s))),
        }
    }
}

impl ToString for ResumeState {
    fn to_string(&self) -> String {
        let resume_state = match *self {
            ResumeState::PlaybackStart => "PLAYBACK_START",
            ResumeState::PlaybackPause => "PLAYBACK_PAUSE",
        };

        resume_state.to_owned()
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
    /// Unique ID for the playback of this specific session. This ID is set by the receiver at LOAD
    /// and can be used to identify a specific instance of a playback. For example, two playbacks of
    /// "Wish you were here" within the same session would each have a unique mediaSessionId.
    pub media_session_id: i32,
    /// Full description of the content that is being played back. Only be returned in a status
    /// messages if the Media has changed.
    pub media: Option<Media>,
    /// Indicates whether the media time is progressing, and at what rate. This is independent of
    /// the player state since the media time can stop in any state. 1.0 is regular time, 0.5 is
    /// slow motion.
    pub playback_rate: f32,
    /// Describes the state of the player.
    pub player_state: PlayerState,
    /// If the player_state is IDLE and the reason it became IDLE is known, this property is
    /// provided. If the player is IDLE because it just started, this property will not be provided.
    /// If the player is in any other state this property should not be provided.
    pub idle_reason: Option<IdleReason>,
    /// The current position of the media player since the beginning of the content, in seconds.
    /// If this a live stream content, then this field represents the time in seconds from the
    /// beginning of the event that should be known to the player.
    pub current_time: Option<f32>,
    /// Flags describing which media commands the media player supports:
    /// * `1` `Pause`;
    /// * `2` `Seek`;
    /// * `4` `Stream volume`;
    /// * `8` `Stream mute`;
    /// * `16` `Skip forward`;
    /// * `32` `Skip backward`.
    /// Combinations are described as summations; for example, Pause+Seek+StreamVolume+Mute == 15.
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

    /// Pauses playback of the current content. Triggers a STATUS event notification to all sender
    /// applications.
    pub fn pause<S>(&self, destination: S, media_session_id: i32)
        -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = try!(serde_json::to_string(
            &proxies::media::PlaybackGenericRequest {
                request_id: 3000,
                media_session_id: media_session_id,
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

    /// Begins playback of the content that was loaded with the load call, playback is continued
    /// from the current time position.
    pub fn play<S>(&self, destination: S, media_session_id: i32)
        -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = try!(serde_json::to_string(
            &proxies::media::PlaybackGenericRequest {
                request_id: 4000,
                media_session_id: media_session_id,
                typ: MESSAGE_TYPE_PLAY.to_owned(),
                custom_data: proxies::media::CustomData::new(),
            }));

        MessageManager::send(&mut *self.writer.borrow_mut(), CastMessage {
            namespace: CHANNEL_NAMESPACE.to_owned(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        })
    }

    /// Stops playback of the current content. Triggers a STATUS event notification to all sender
    /// applications. After this command the content will no longer be loaded and the
    /// media_session_id is invalidated.
    pub fn stop<S>(&self, destination: S, media_session_id: i32)
        -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = try!(serde_json::to_string(
            &proxies::media::PlaybackGenericRequest {
                request_id: 5000,
                media_session_id: media_session_id,
                typ: MESSAGE_TYPE_STOP.to_owned(),
                custom_data: proxies::media::CustomData::new(),
            }));

        MessageManager::send(&mut *self.writer.borrow_mut(), CastMessage {
            namespace: CHANNEL_NAMESPACE.to_owned(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        })
    }

    /// Sets the current position in the stream. Triggers a STATUS event notification to all sender
    /// applications. If the position provided is outside the range of valid positions for the
    /// current content, then the player should pick a valid position as close to the requested
    /// position as possible.
    pub fn seek<S>(&self, destination: S, media_session_id: i32, current_time: Option<f32>,
                   resume_state: Option<ResumeState>)
        -> Result<(), Error> where S: Into<Cow<'a, str>> {
        let payload = try!(serde_json::to_string(
            &proxies::media::PlaybackSeekRequest {
                request_id: 6000,
                media_session_id: media_session_id,
                typ: MESSAGE_TYPE_SEEK.to_owned(),
                current_time: current_time,
                resume_state: resume_state.map(|s| s.to_string()),
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
