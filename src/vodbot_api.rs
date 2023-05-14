// Structs for handling VodBot specifically generated data

// use crate::twitch_api;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::twitch_api::{
    TwitchClip, TwitchPlaybackAccessToken, TwitchUser, TwitchVideo, TwitchVideoComment,
    TwitchVideoMoment,
};

// Pull related data

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VodChapter {
    pub description: String,
    pub position: usize,
    pub duration: usize,
    // pub r#type: VodChapterType,
}
impl VodChapter {
    pub fn from_data(n: &TwitchVideoMoment) -> Self {
        Self {
            description: n.description.to_owned(),
            position: n.position_milliseconds / 1000,
            duration: n.duration_milliseconds / 1000,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Vod {
    pub id: String,

    pub streamer_id: String,
    pub streamer_login: String,
    pub streamer_name: String,

    pub game_id: String,
    pub game_name: String,

    pub title: String,
    pub created_at: String,
    pub chapters: Vec<VodChapter>,
    pub duration: usize,
    pub has_chat: bool,
}
impl Vod {
    pub fn from_data(u: &TwitchUser, v: &TwitchVideo, c: Vec<VodChapter>) -> Self {
        Self {
            id: v.id.to_owned(),
            streamer_id: u.id.clone(),
            streamer_login: u.login.clone(),
            streamer_name: u.display_name.clone(),
            game_id: v
                .game
                .as_ref()
                .map(|f| f.id.to_owned())
                .unwrap_or("".to_owned()),
            game_name: v
                .game
                .as_ref()
                .map(|f| f.name.to_owned())
                .unwrap_or("".to_owned()),
            title: v.title.to_owned(),
            created_at: v.created_at.to_owned(),
            chapters: c,
            duration: v.length_seconds,
            has_chat: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub id: String,
    pub slug: String,

    pub streamer_id: String,
    pub streamer_login: String,
    pub streamer_name: String,

    pub clipper_id: String,
    pub clipper_login: String,
    pub clipper_name: String,

    pub game_id: String,
    pub game_name: String,

    pub title: String,
    pub created_at: String,
    pub view_count: usize,
    pub duration: usize,
    pub offset: usize,

    pub vod_id: String,
    // pub url: String,
}
impl Clip {
    pub fn from_data(u: &TwitchUser, n: &TwitchClip) -> Self {
        Self {
            id: n.id.to_owned(),
            slug: n.slug.to_owned(),
            streamer_id: u.id.clone(),
            streamer_login: u.login.clone(),
            streamer_name: u.display_name.clone(),
            clipper_id: n
                .curator
                .as_ref()
                .map(|f| f.id.to_owned())
                .unwrap_or("".to_owned()),
            clipper_login: n
                .curator
                .as_ref()
                .map(|f| f.login.to_owned())
                .unwrap_or("".to_owned()),
            clipper_name: n
                .curator
                .as_ref()
                .map(|f| f.display_name.to_owned())
                .unwrap_or("".to_owned()),
            game_id: n
                .game
                .as_ref()
                .map(|f| f.id.to_owned())
                .unwrap_or("".to_owned()),
            game_name: n
                .game
                .as_ref()
                .map(|f| f.name.to_owned())
                .unwrap_or("".to_owned()),
            title: n.title.to_owned(),
            created_at: n.created_at.to_owned(),
            duration: n.duration_seconds,
            offset: n.video_offset_secconds.unwrap_or(0),
            view_count: n.view_count,
            vod_id: n
                .video
                .as_ref()
                .map(|f| f.id.to_owned())
                .unwrap_or("".to_owned()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: String,
    pub login: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub user_name: String,
    pub color: String,
    pub offset: usize,
    pub msg: String,
}
impl ChatMessage {
    pub fn from_data(n: &TwitchVideoComment) -> Self {
        let f = &n.message.fragments;
        Self {
            user_name: n.commenter.display_name.to_owned(),
            color: n.message.user_color.to_owned().unwrap_or("".to_owned()),
            offset: n.content_offset_seconds,
            msg: f
                .iter()
                .map(|f| {
                    f.mention
                        .as_ref()
                        .map(|f| format!("@{} ", f.display_name))
                        .unwrap_or("".to_owned())
                        .to_owned()
                        + &f.text
                })
                .collect(), // ::<Vec<String>>().join(" "),
        }
    }
}

// Stage related data

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoSlice {
    pub video_id: String,
    pub ss: String,
    pub to: String,
    pub filepath: PathBuf,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct ThumbnailData {
//     pub heads: Vec<String>,
//     pub game: String,
//     pub text: String,
//     pub video_slice_idx: usize,
//     pub timestamp: String,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StageData {
    pub title: String,
    pub description: String,
    pub streamers: Vec<String>,
    // pub thumbnail: Option<ThumbnailData>,
    pub slices: Vec<VideoSlice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackAccessToken {
    pub value: String,
    pub signature: String,
}
impl PlaybackAccessToken {
    pub fn from_data(u: &TwitchPlaybackAccessToken) -> Self {
        Self {
            value: u.value.to_owned(),
            signature: u.signature.to_owned(),
        }
    }
}
