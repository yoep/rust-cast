use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use serde_json;

use cast::proxies;
use errors::Error;
use message_manager::{CastMessage, CastMessagePayload, MessageManager};

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.media";

const MESSAGE_TYPE_LOAD: &'static str = "LOAD";
const MESSAGE_TYPE_MEDIA_STATUS: &'static str = "MEDIA_STATUS";
const MESSAGE_TYPE_LOAD_CANCELLED: &'static str = "LOAD_CANCELLED";

/// Describes the way cast device should stream content.
pub enum StreamType {
    /// This variant allows cast device to automatically choose whatever way it's most comfortable
    /// with.
    None,
    /// Cast device should buffer some portion of the content and only then start streaming.
    Buffered,
    /// Cast device should display content as soon as it gets any portion of it.
    Live,
}

/// Represents all currently supported incoming messages that media channel can handle.
#[derive(Debug)]
pub enum MediaResponse {
    /// Status of the currently active media.
    MediaStatus(proxies::media::MediaStatusReply),
    /// Information about cancelled media.
    LoadCancelled(proxies::media::LoadCancelledReply),
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

    pub fn load<S>(&self, destination: S, session_id: S, content_id: S, content_type: S,
                   stream_type: StreamType) -> Result<(), Error> where S: Into<Cow<'a, str>> {

        let stream_type_string = match stream_type {
            StreamType::None => "NONE",
            StreamType::Buffered => "BUFFERED",
            StreamType::Live => "LIVE",
        };

        let payload = try!(serde_json::to_string(
            &proxies::media::MediaRequest {
                request_id: 1000,
                session_id: session_id.into().to_string(),
                typ: MESSAGE_TYPE_LOAD.to_owned(),

                media: proxies::media::Media {
                    content_id: content_id.into().to_string(),
                    stream_type: stream_type_string.to_string(),
                    content_type: content_type.into().to_string(),
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
