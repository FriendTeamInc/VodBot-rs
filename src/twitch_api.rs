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
    pub locations: Vec<TwitchResponseErrorLocation>,
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
    pub has_previous_page: bool,
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
pub enum TwitchVideoBroadcastType {
    ARCHIVE,
    HIGHLIGHT,
    UPLOAD,
    PREMIERE_UPLOAD,
    PAST_PREMIERE,
}

#[derive(Debug, Deserialize)]
pub enum TwitchVideoStatus {
    RECORDING,
    UNPROCESSED,
    CREATED,
    UPLOADING,
    PENDING_TRANSCODE,
    TRANSCODING,
    FAILED,
    RECORDED,
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
    pub creator: TwitchUserVideoUser,
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
    pub id: Option<String>,
    pub login: Option<String>,
    pub display_name: Option<String>,
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
    user(TwitchUser),
    video(()),
    clip(()),
    video_playback_access_token(TwitchResponsePlaybackAccessToken),
}

#[derive(Debug, Deserialize)]
pub struct TwitchResponse {
    pub errors: Option<Vec<TwitchResponseError>>,
    // extensions: TwitchResponseExtensions,
    pub data: Option<TwitchResponseData>,
}
