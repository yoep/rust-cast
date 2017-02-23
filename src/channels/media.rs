use std::borrow::Cow;
use std::str::FromStr;
use std::string::ToString;
use std::io::{Read, Write};
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
const MESSAGE_TYPE_LOAD_FAILED: &'static str = "LOAD_FAILED";
const MESSAGE_TYPE_INVALID_PLAYER_STATE: &'static str = "INVALID_PLAYER_STATE";
const MESSAGE_TYPE_INVALID_REQUEST: &'static str = "INVALID_REQUEST";

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
            "BUFFERED" | "buffered"  => Ok(StreamType::Buffered),
            "LIVE" | "live" => Ok(StreamType::Live),
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

        stream_type.to_string()
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

impl ToString for PlayerState {
    fn to_string(&self) -> String {
        let player_state = match *self {
            PlayerState::Idle => "IDLE",
            PlayerState::Playing => "PLAYING",
            PlayerState::Buffering => "BUFFERING",
            PlayerState::Paused => "PAUSED",
        };

        player_state.to_string()
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

        resume_state.to_string()
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
    /// Unique id of the request that requested the status.
    pub request_id: i32,
    /// Detailed status of every media status entry.
    pub entries: Vec<StatusEntry>,
}

/// Detailed status of the media artifact with respect to the session.
#[derive(Debug)]
pub struct StatusEntry {
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

/// Describes the load cancelled error.
#[derive(Debug)]
pub struct LoadCancelled {
    /// Unique id of the request that caused this error.
    pub request_id: i32,
}

/// Describes the load failed error.
#[derive(Debug)]
pub struct LoadFailed {
    /// Unique id of the request that caused this error.
    pub request_id: i32,
}

/// Describes the invalid player state error.
#[derive(Debug)]
pub struct InvalidPlayerState {
    /// Unique id of the request that caused this error.
    pub request_id: i32,
}

/// Describes the invalid request error.
#[derive(Debug)]
pub struct InvalidRequest {
    /// Unique id of the invalid request.
    pub request_id: i32,
    /// Description of the invalid request reason if available.
    pub reason: Option<String>,
}

/// Represents all currently supported incoming messages that media channel can handle.
#[derive(Debug)]
pub enum MediaResponse {
    /// Statuses of the currently active media.
    Status(Status),
    /// Sent when the load request was cancelled (a second load request was received).
    LoadCancelled(LoadCancelled),
    /// Sent when the load request failed. The player state will be IDLE.
    LoadFailed(LoadFailed),
    /// Sent when the request by the sender can not be fulfilled because the player is not in a
    /// valid state. For example, if the application has not created a media element yet.
    InvalidPlayerState(InvalidPlayerState),
    /// Error indicating that request is not valid.
    InvalidRequest(InvalidRequest),
    /// Used every time when channel can't parse the message. Associated data contains `type` string
    /// field and raw JSON data returned from cast device.
    NotImplemented(String, serde_json::Value),
}

pub struct MediaChannel<'a, W> where W: Read + Write {
    sender: Cow<'a, str>,
    message_manager: Rc<MessageManager<W>>,
}

impl<'a, W> MediaChannel<'a, W> where W: Read + Write {
    pub fn new<S>(sender: S, message_manager: Rc<MessageManager<W>>)
        -> MediaChannel<'a, W> where S: Into<Cow<'a, str>> {
        MediaChannel {
            sender: sender.into(),
            message_manager: message_manager,
        }
    }

    /// Retrieves status of the cast device media session.
    ///
    /// # Arguments
    ///
    /// * `destination` - `protocol` identifier of specific app media session;
    /// * `media_session_id` - Media session ID of the media for which the media status should be
    /// returned. If none is provided, then the status for all media session IDs will be provided.
    ///
    /// # Return value
    ///
    /// Returned `Result` should consist of either `Status` instance or an `Error`.
    pub fn get_status<S>(&self, destination: S, media_session_id: Option<i32>)
        -> Result<Status, Error> where S: Into<Cow<'a, str>> {
        let request_id = self.message_manager.generate_request_id();

        let payload = try!(serde_json::to_string(
            &proxies::media::GetStatusRequest {
                typ: MESSAGE_TYPE_GET_STATUS.to_string(),
                request_id: request_id,
                media_session_id: media_session_id,
            }));

        try!(self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        }));

        self.message_manager.receive_find_map(|message| {
            if !self.can_handle(message) {
                return Ok(None);
            }

            match try!(self.parse(message)) {
                MediaResponse::Status(status) => {
                    if status.request_id == request_id {
                        return Ok(Some(status));
                    }
                },
                MediaResponse::InvalidRequest(error) => {
                    if error.request_id == request_id {
                        return Err(Error::Internal(
                            format!("Invalid request ({}).",
                                    error.reason.unwrap_or("Unknown".to_string())))
                        );
                    }
                },
                _ => {}
            }

            return Ok(None);
        })
    }

    /// Loads provided media to the application.
    ///
    /// # Arguments
    /// * `destination` - `protocol` of the application to load media with (e.g. `web-1`);
    /// * `session_id` - Current session identifier of the player application;
    /// * `media` - `Media` instance that describes the media we'd like to load.
    ///
    /// # Return value
    ///
    /// Returned `Result` should consist of either `Status` instance or an `Error`.
    pub fn load<S>(&self, destination: S, session_id: S, media: Media)
        -> Result<Status, Error> where S: Into<Cow<'a, str>> {
        let request_id = self.message_manager.generate_request_id();

        let payload = try!(serde_json::to_string(
            &proxies::media::MediaRequest {
                request_id: request_id,
                session_id: session_id.into().to_string(),
                typ: MESSAGE_TYPE_LOAD.to_string(),

                media: proxies::media::Media {
                    content_id: media.content_id.clone(),
                    stream_type: media.stream_type.to_string(),
                    content_type: media.content_type.clone(),
                    duration: media.duration,
                },

                current_time: 0_f64,
                autoplay: true,
                custom_data: proxies::media::CustomData::new(),
            }));

        try!(self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        }));

        // Once media is loaded cast receiver device should emit status update event, or load failed
        // event if something went wrong.
        self.message_manager.receive_find_map(|message| {
            if !self.can_handle(message) {
                return Ok(None);
            }

            match try!(self.parse(message)) {
                MediaResponse::Status(status) => {
                    if status.request_id == request_id {
                        return Ok(Some(status));
                    }

                    // [WORKAROUND] In some cases we don't receive response (e.g. from YouTube app),
                    // so let's just wait for the response with the media we're interested in and
                    // return it.
                    let has_media = {
                        status.entries.iter().find(|ref entry| {
                            if let Some(ref loaded_media) = entry.media {
                                return loaded_media.content_id == media.content_id;
                            }

                            false
                        }).is_some()
                    };

                    if has_media {
                        return Ok(Some(status));
                    }
                },
                MediaResponse::LoadFailed(error) => {
                    if error.request_id == request_id {
                        return Err(Error::Internal("Failed to load media.".to_string()));
                    }
                },
                MediaResponse::LoadCancelled(error) => {
                    if error.request_id == request_id {
                        return Err(
                            Error::Internal("Load cancelled by another request.".to_string()));
                    }
                },
                MediaResponse::InvalidPlayerState(error) => {
                    if error.request_id == request_id {
                        return Err(Error::Internal(
                            "Load failed because of invalid player state.".to_string()));
                    }
                },
                _ => {}
            }

            return Ok(None);
        })
    }

    /// Pauses playback of the current content. Triggers a STATUS event notification to all sender
    /// applications.
    ///
    /// # Arguments
    ///
    /// * `destination` - `protocol` of the media application (e.g. `web-1`);
    /// * `media_session_id` - ID of the media session to be paused.
    ///
    /// # Return value
    ///
    /// Returned `Result` should consist of either `Status` instance or an `Error`.
    pub fn pause<S>(&self, destination: S, media_session_id: i32)
        -> Result<StatusEntry, Error> where S: Into<Cow<'a, str>> {
        let request_id = self.message_manager.generate_request_id();

        let payload = try!(serde_json::to_string(
            &proxies::media::PlaybackGenericRequest {
                request_id: request_id,
                media_session_id: media_session_id,
                typ: MESSAGE_TYPE_PAUSE.to_string(),
                custom_data: proxies::media::CustomData::new(),
            }));

        try!(self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        }));

        self.receive_status_entry(request_id, media_session_id)
    }

    /// Begins playback of the content that was loaded with the load call, playback is continued
    /// from the current time position.
    ///
    /// # Arguments
    ///
    /// * `destination` - `protocol` of the media application (e.g. `web-1`);
    /// * `media_session_id` - ID of the media session to be played.
    ///
    /// # Return value
    ///
    /// Returned `Result` should consist of either `Status` instance or an `Error`.
    pub fn play<S>(&self, destination: S, media_session_id: i32)
        -> Result<StatusEntry, Error> where S: Into<Cow<'a, str>> {
        let request_id = self.message_manager.generate_request_id();

        let payload = try!(serde_json::to_string(
            &proxies::media::PlaybackGenericRequest {
                request_id: request_id,
                media_session_id: media_session_id,
                typ: MESSAGE_TYPE_PLAY.to_string(),
                custom_data: proxies::media::CustomData::new(),
            }));

        try!(self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        }));

        self.receive_status_entry(request_id, media_session_id)
    }

    /// Stops playback of the current content. Triggers a STATUS event notification to all sender
    /// applications. After this command the content will no longer be loaded and the
    /// media_session_id is invalidated.
    ///
    /// # Arguments
    ///
    /// * `destination` - `protocol` of the media application (e.g. `web-1`);
    /// * `media_session_id` - ID of the media session to be stopped.
    ///
    /// # Return value
    ///
    /// Returned `Result` should consist of either `Status` instance or an `Error`.
    pub fn stop<S>(&self, destination: S, media_session_id: i32)
        -> Result<StatusEntry, Error> where S: Into<Cow<'a, str>> {
        let request_id = self.message_manager.generate_request_id();

        let payload = try!(serde_json::to_string(
            &proxies::media::PlaybackGenericRequest {
                request_id: request_id,
                media_session_id: media_session_id,
                typ: MESSAGE_TYPE_STOP.to_string(),
                custom_data: proxies::media::CustomData::new(),
            }));

        try!(self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        }));

        self.receive_status_entry(request_id, media_session_id)
    }

    /// Sets the current position in the stream. Triggers a STATUS event notification to all sender
    /// applications. If the position provided is outside the range of valid positions for the
    /// current content, then the player should pick a valid position as close to the requested
    /// position as possible.
    ///
    /// # Arguments
    ///
    /// * `destination` - `protocol` of the media application (e.g. `web-1`);
    /// * `media_session_id` - ID of the media session to seek in;
    /// * `current_time` - Time in seconds to seek to.
    ///
    /// # Return value
    ///
    /// Returned `Result` should consist of either `Status` instance or an `Error`.
    pub fn seek<S>(&self, destination: S, media_session_id: i32, current_time: Option<f32>,
                   resume_state: Option<ResumeState>)
        -> Result<StatusEntry, Error> where S: Into<Cow<'a, str>> {
        let request_id = self.message_manager.generate_request_id();

        let payload = try!(serde_json::to_string(
            &proxies::media::PlaybackSeekRequest {
                request_id: request_id,
                media_session_id: media_session_id,
                typ: MESSAGE_TYPE_SEEK.to_string(),
                current_time: current_time,
                resume_state: resume_state.map(|s| s.to_string()),
                custom_data: proxies::media::CustomData::new(),
            }));

        try!(self.message_manager.send(CastMessage {
            namespace: CHANNEL_NAMESPACE.to_string(),
            source: self.sender.to_string(),
            destination: destination.into().to_string(),
            payload: CastMessagePayload::String(payload),
        }));

        self.receive_status_entry(request_id, media_session_id)
    }

    pub fn can_handle(&self, message: &CastMessage) -> bool {
        message.namespace == CHANNEL_NAMESPACE
    }

    pub fn parse(&self, message: &CastMessage) -> Result<MediaResponse, Error> {
        let reply = match message.payload {
            CastMessagePayload::String(ref payload) => try!(
                serde_json::from_str::<serde_json::Value>(payload)),
            _ => return Err(Error::Internal("Binary payload is not supported!".to_string())),
        };

        let message_type = reply.as_object()
            .and_then(|object| object.get("type"))
            .and_then(|property| property.as_str())
            .unwrap_or("")
            .to_string();

        let response = match message_type.as_ref() {
            MESSAGE_TYPE_MEDIA_STATUS => {
                let reply: proxies::media::StatusReply = try!(
                    serde_json::value::from_value(reply));

                let statuses_entries = reply.status.iter().map(|ref x| {
                    StatusEntry {
                        media_session_id: x.media_session_id,
                        media: x.media.as_ref().map(|ref m| Media {
                            content_id: m.content_id.to_string(),
                            stream_type: StreamType::from_str(m.stream_type.as_ref()).unwrap(),
                            content_type: m.content_type.to_string(),
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

                MediaResponse::Status(Status {
                    request_id: reply.request_id,
                    entries: statuses_entries.collect::<Vec<StatusEntry>>(),
                })
            },
            MESSAGE_TYPE_LOAD_CANCELLED => {
                let reply: proxies::media::LoadCancelledReply = try!(
                    serde_json::value::from_value(reply));

                MediaResponse::LoadCancelled(LoadCancelled {
                    request_id: reply.request_id,
                })
            },
            MESSAGE_TYPE_LOAD_FAILED => {
                let reply: proxies::media::LoadFailedReply = try!(
                    serde_json::value::from_value(reply));

                MediaResponse::LoadFailed(LoadFailed {
                    request_id: reply.request_id,
                })
            },
            MESSAGE_TYPE_INVALID_PLAYER_STATE => {
                let reply: proxies::media::InvalidPlayerStateReply = try!(
                    serde_json::value::from_value(reply));

                MediaResponse::InvalidPlayerState(InvalidPlayerState {
                    request_id: reply.request_id,
                })
            },
            MESSAGE_TYPE_INVALID_REQUEST => {
                let reply: proxies::media::InvalidRequestReply = try!(
                    serde_json::value::from_value(reply));

                MediaResponse::InvalidRequest(InvalidRequest {
                    request_id: reply.request_id,
                    reason: reply.reason,
                })
            },
            _ => MediaResponse::NotImplemented(message_type.to_string(), reply),
        };

        Ok(response)
    }

    /// Waits for the status entry with specified `request_id` and `media_session_id`. This method
    /// is very handy for the media playback methods where particular `StatusEntry` is required.
    ///
    /// # Arguments
    ///
    /// * `request_id` - ID of the request that caused status entry to be broadcasted.
    /// * `media_session_id` - ID of the media session to receive.
    ///
    /// # Return value
    ///
    /// Returned `Result` should consist of either `Status` instance or an `Error`.
    fn receive_status_entry(&self, request_id: i32, media_session_id: i32)
        -> Result<StatusEntry, Error> {
        self.message_manager.receive_find_map(|message| {
            if !self.can_handle(message) {
                return Ok(None);
            }

            match try!(self.parse(message)) {
                MediaResponse::Status(mut status) => {
                    if status.request_id == request_id {
                        let position = status.entries.iter().position(|e| {
                            e.media_session_id == media_session_id
                        });

                        return Ok(position.and_then(|position| {
                            Some(status.entries.remove(position))
                        }));
                    }
                },
                MediaResponse::InvalidPlayerState(error) => {
                    if error.request_id == request_id {
                        return Err(Error::Internal(
                            "Request failed because of invalid player state.".to_string()));
                    }
                },
                MediaResponse::InvalidRequest(error) => {
                    if error.request_id == request_id {
                        return Err(Error::Internal(
                            format!("Invalid request ({}).",
                                    error.reason.unwrap_or("Unknown".to_string())))
                        );
                    }
                },
                _ => {}
            }

            return Ok(None);
        })
    }
}
