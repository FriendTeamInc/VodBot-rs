// Twitch's GQL API has some whacky output structures.
// Defining them here like they are in the API makes it just easier to work with.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TwitchResponseErrorLocation {
    line: usize,
    column: usize,
}

#[derive(Debug, Deserialize)]
struct TwitchResponseError {
    message: String,
    locations: Vec<TwitchResponseErrorLocation>,
}

#[derive(Debug, Deserialize)]
struct TwitchResponseExtensions {
    #[serde(rename = "durationMilliseconds")]
    duration_milliseconds: usize,
    #[serde(rename = "requestID")]
    request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TwitchGame {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserStream {
    id: String,
    title: String,
    r#type: String,
    viewers_count: usize,
    created_at: String,
    // game: TwitchGame,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserRoles {
    is_affiliate: bool,
    is_partner: bool,
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
    id: String,
    login: String,
    display_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserVideoNode {
    id: String,
    title: String,
    published_at: String,
    broadcast_type: TwitchVideoBroadcastType,
    status: TwitchVideoStatus,
    length_seconds: usize,
    game: Option<TwitchGame>,
    creator: TwitchUserVideoUser,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserVideoEdge {
    cursor: Option<String>,
    node: TwitchUserVideoNode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUserVideoConnection {
    total_count: usize,
    edges: Vec<TwitchUserVideoEdge>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchUser {
    id: Option<String>,
    login: Option<String>,
    display_name: Option<String>,

    roles: Option<TwitchUserRoles>,
    stream: Option<TwitchUserStream>,

    videos: Option<TwitchUserVideoConnection>,
}

#[derive(Debug, Deserialize)]
pub struct TwitchResponsePlaybackAccessToken {
    value: String,
    signature: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TwitchResponseData {
    user(Option<TwitchUser>),
    video(Option<()>),
    clip(Option<()>),
    video_playback_access_token(Option<TwitchResponsePlaybackAccessToken>),
}

#[derive(Debug, Deserialize)]
pub struct TwitchResponse {
    errors: Option<Vec<TwitchResponseError>>,
    extensions: TwitchResponseExtensions,
    data: Option<TwitchResponseData>,
}
