// Twitch's GQL API has some whacky output structures.
// Defining them here like they are in the API makes it just easier to work with.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchResponseErrorLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Deserialize)]
pub struct TwitchResponseError {
    pub message: String,
    pub locations: Option<Vec<TwitchResponseErrorLocation>>,
}

#[derive(Debug, Deserialize)]
pub struct TwitchResponseExtensions {
    #[serde(rename = "durationMilliseconds")]
    pub duration_milliseconds: usize,
    #[serde(rename = "requestID")]
    pub request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TwitchGame {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchPageInfo {
    pub has_next_page: bool,
    // pub has_previous_page: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserStream {
    pub id: String,
    pub title: String,
    pub r#type: String,
    pub viewers_count: usize,
    pub created_at: String,
    pub game: TwitchGame,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserRoles {
    pub is_affiliate: bool,
    pub is_partner: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TwitchVideoBroadcastType {
    Archive,
    Highlight,
    Upload,
    PremiereUpload,
    PastPremiere,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TwitchVideoStatus {
    Recording,
    Unprocessed,
    Created,
    Uploading,
    PendingTranscode,
    Transcoding,
    Failed,
    Recorded,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserVideoUser {
    pub id: String,
    pub login: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserVideoNode {
    pub id: String,
    pub title: String,
    pub published_at: String,
    pub broadcast_type: TwitchVideoBroadcastType,
    pub status: TwitchVideoStatus,
    pub length_seconds: usize,
    pub game: Option<TwitchGame>,
    // pub creator: TwitchUserVideoUser,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserVideoEdge {
    pub cursor: Option<String>,
    pub node: TwitchUserVideoNode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserVideoConnection {
    pub page_info: TwitchPageInfo,
    pub total_count: usize,
    pub edges: Vec<TwitchUserVideoEdge>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUser {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub roles: Option<TwitchUserRoles>,
    pub stream: Option<TwitchUserStream>,
    pub videos: Option<TwitchUserVideoConnection>,
}

#[derive(Debug, Deserialize)]
pub struct TwitchResponsePlaybackAccessToken {
    pub value: String,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TwitchResponseData {
    User(TwitchUser),
    Video(()),
    Clip(()),
    VideoPlaybackAccessToken(TwitchResponsePlaybackAccessToken),
}

#[derive(Debug, Deserialize)]
pub struct TwitchResponse {
    pub errors: Option<Vec<TwitchResponseError>>,
    // extensions: TwitchResponseExtensions,
    pub data: Option<TwitchResponseData>,
}
