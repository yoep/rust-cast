use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use serde_json;

use errors::Error;
use message_manager::{CastMessage, CastMessagePayload, MessageManager};

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.media";

const MESSAGE_TYPE_LOAD: &'static str = "LOAD";
const MESSAGE_TYPE_MEDIA_STATUS: &'static str = "MEDIA_STATUS";
const MESSAGE_TYPE_LOAD_CANCELLED: &'static str = "LOAD_CANCELLED";

pub enum StreamType {
    None,
    Buffered,
    Live,
}

#[derive(Serialize, Debug)]
pub struct MediaRequest<'a> {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="sessionId")]
    pub session_id: Cow<'a, str>,

    #[serde(rename="type")]
    pub typ: String,

    pub media: Media<'a>,

    #[serde(rename="currentTime")]
    pub current_time: f64,

    #[serde(rename="customData")]
    pub custom_data: CustomData,

    pub autoplay: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media<'a> {
    #[serde(rename="contentId")]
    pub content_id: Cow<'a, str>,

    #[serde(rename="streamType", default)]
    pub stream_type: Cow<'a, str>,

    #[serde(rename="contentType")]
    pub content_type: Cow<'a, str>,
}

#[derive(Serialize, Debug)]
pub struct CustomData {
    #[serde(skip_serializing)]
    private: (),
}

impl CustomData {
    pub fn new() -> CustomData {
        CustomData { private: () }
    }
}

#[derive(Deserialize, Debug)]
pub struct MediaStatus<'a> {
    #[serde(default)]
    pub media: Option<Media<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct MediaStatusReply<'a> {
    #[serde(rename="requestId", default)]
    pub request_id: i32,

    #[serde(rename="type")]
    pub typ: String,

    pub status: Vec<MediaStatus<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct LoadCancelledReply {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="type")]
    typ: String,
}

#[derive(Debug)]
pub enum MediaResponse<'a> {
    MediaStatus(MediaStatusReply<'a>),
    LoadCancelled(LoadCancelledReply),
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

    pub fn load<S>(&self, destination: S, session_id: S, content_id: S, content_type: S,
                   stream_type: StreamType) -> Result<(), Error> where S: Into<Cow<'a, str>> {

        let stream_type_string = match stream_type {
            StreamType::None => "NONE",
            StreamType::Buffered => "BUFFERED",
            StreamType::Live => "LIVE",
        };

        let payload = try!(serde_json::to_string(
            &MediaRequest {
                request_id: 1,
                session_id: session_id.into(),
                typ: MESSAGE_TYPE_LOAD.to_owned(),

                media: Media {
                    content_id: content_id.into(),
                    stream_type: stream_type_string.into(),
                    content_type: content_type.into(),
                },

                current_time: 0_f64,
                autoplay: true,
                custom_data: CustomData::new(),
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
            MESSAGE_TYPE_MEDIA_STATUS => MediaResponse::MediaStatus(
                try!(serde_json::value::from_value(reply))),
            MESSAGE_TYPE_LOAD_CANCELLED => MediaResponse::LoadCancelled(
                try!(serde_json::value::from_value(reply))),
            _ => MediaResponse::NotImplemented(message_type.to_owned(), reply),
        };

        Ok(response)
    }
}
