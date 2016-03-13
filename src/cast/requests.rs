#[derive(Serialize, Deserialize, Debug)]
pub struct GenericRequest {
    #[serde(rename="type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppLaunchRequest {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="type")]
    pub typ: String,

    #[serde(rename="appId")]
    pub app_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaRequest {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="sessionId")]
    pub session_id: String,

    #[serde(rename="type")]
    pub typ: String,

    pub media: Media,

    #[serde(rename="currentTime")]
    pub current_time: f64,

    #[serde(rename="customData")]
    pub custom_data: CustomData,

    pub autoplay: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    #[serde(rename="contentId")]
    pub content_id: String,

    #[serde(rename="streamType")]
    pub stream_type: String,

    #[serde(rename="contentType")]
    pub content_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomData {
    #[serde(skip_serializing)]
    private: (),
}

impl CustomData {
    pub fn new() -> CustomData {
        CustomData { private: () }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiverResponse {
    #[serde(rename="requestId")]
    pub request_id: i32,

    #[serde(rename="type")]
    pub typ: String,

    pub status: ReceiverStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiverStatus {
    #[serde(default)]
    pub applications: Vec<Application>,

    #[serde(rename="isActiveInput", default)]
    pub is_active_input: bool,

    #[serde(rename="isStandBy", default)]
    pub is_stand_by: bool,

    pub volume: ReceiverVolume,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct AppNamespace {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiverVolume {
    pub level: f64,
    pub muted: bool,
}
