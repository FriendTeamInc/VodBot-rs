// Twitch's GQL API has some whacky output structures.
// Defining them here like they are in the API makes it just easier to work with.

use std::collections::HashMap;

use serde::Deserialize;

pub trait TwitchData {}
pub trait TwitchNode {}

structstruck::strike! {
    #[strikethrough[derive(Debug, Deserialize, Clone)]]
    #[serde(rename_all = "camelCase")]
    pub struct TwitchVideo {
        pub id: String,
        pub title: String,
        pub created_at: String,
        pub broadcast_type: pub enum TwitchVideoBroadcastType {
            #![serde(rename_all = "SCREAMING_SNAKE_CASE")]
            Archive,
            Highlight,
            Upload,
            PremiereUpload,
            PastPremiere,
        },
        pub status: pub enum TwitchVideoStatus {
            #![serde(rename_all = "SCREAMING_SNAKE_CASE")]
            Recording,
            Unprocessed,
            Created,
            Uploading,
            PendingTranscode,
            Transcoding,
            Failed,
            Recorded,
        },
        pub length_seconds: usize,
        pub game: Option<pub struct TwitchGame {
            #![serde(rename_all = "camelCase")]
            pub id: String,
            pub name: String,
        }>,
        // pub creator: TwitchUserVideoUser,
        pub comments: Option<TwitchConnection<TwitchVideoComment>>,
        pub moments: Option<TwitchConnection<pub struct TwitchVideoMoment {
            #![serde(rename_all = "camelCase")]
            pub description: String,
            pub position_milliseconds: usize,
            pub duration_milliseconds: usize,
        }>>,
    }
}
impl TwitchData for TwitchVideo {}
impl TwitchNode for TwitchVideo {}
impl TwitchNode for TwitchVideoMoment {}

structstruck::strike! {
    #[strikethrough[derive(Debug, Deserialize, Clone)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct TwitchVideoComment {
        pub content_offset_seconds: usize,
        pub commenter: pub struct TwitchVideoCommentUser {
            pub display_name: String,
        },
        pub message: 
        pub struct TwitchVideoCommentMessage {
            pub user_color: Option<String>,
            pub fragments: Vec<pub struct TwitchVideoCommentFragment {
                pub text: String,
                pub mention: Option<TwitchVideoCommentUser>,
            }>,
        },
    }
}
impl TwitchNode for TwitchVideoComment {}

structstruck::strike! {
    #[strikethrough[derive(Debug, Deserialize, Clone)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct TwitchClip {
        pub id: String,
        pub slug: String,
        pub title: String,
        pub created_at: String,
        pub view_count: usize,
        pub duration_seconds: usize,
        pub video_offset_secconds: Option<usize>,
        pub video: Option<pub struct TwitchClipVideoSource {
            pub id: String,
        }>,
        pub game: Option<TwitchGame>,
        pub curator: Option<TwitchUser>,
    }
}
impl TwitchData for TwitchClip {}
impl TwitchNode for TwitchClip {}

structstruck::strike! {
    #[strikethrough[derive(Debug, Deserialize, Clone)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct TwitchConnection<T: TwitchNode> {
        pub page_info: pub struct TwitchPageInfo {
            pub has_next_page: bool
        },
        pub edges: Vec<pub struct TwitchEdge<T: TwitchNode> {
            pub cursor: Option<String>,
            pub node: T,
        }<T>>,
    }
}

structstruck::strike! {
    #[strikethrough[derive(Debug, Deserialize, Clone)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct TwitchUser {
        pub id: String,
        pub login: String,
        pub display_name: String,
        pub roles: Option<pub struct TwitchUserRoles {
            pub is_affiliate: bool,
            pub is_partner: bool,
        }>,
        pub stream: Option<pub struct TwitchUserStream {
            pub id: String,
            pub title: String,
            pub r#type: String,
            pub viewers_count: usize,
            pub created_at: String,
            pub game: TwitchGame,
        }>,
        pub videos: Option<TwitchConnection<TwitchVideo>>,
        pub clips: Option<TwitchConnection<TwitchClip>>,
    }
}
impl TwitchData for TwitchUser {}

structstruck::strike! {
    #[strikethrough[derive(Debug, Deserialize, Clone)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct TwitchPlaybackAccessTokenToken {
        pub playback_access_token: pub struct TwitchPlaybackAccessToken {
            pub value: String,
            pub signature: String,
        },
    }
}
impl TwitchData for TwitchPlaybackAccessTokenToken {}

structstruck::strike! {
    #[strikethrough[derive(Debug, Deserialize, Clone)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct TwitchResponse<T: TwitchData> {
        pub errors: Option<Vec<pub struct TwitchResponseError {
            pub message: String,
            pub path: Option<Vec<String>>,
            pub locations: Option<Vec<pub struct TwitchResponseErrorLocation {
                pub line: usize,
                pub column: usize,
            }>>
        }>>,
        pub data: Option<HashMap<String, T>>,
    }
}
