// Structs for handling VodBot specifically generated data

// use crate::twitch_api;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::twitch_api::TwitchVideo;

// Pull related data

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VodChapter {
    pub description: String,
    pub position: usize,
    pub duration: usize,
    // pub r#type: VodChapterType,
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
    pub fn from_data(
        user_id: String,
        user_login: String,
        user_name: String,
        v: &TwitchVideo,
        c: Vec<VodChapter>,
    ) -> Vod {
        Vod {
            id: v.id.to_owned(),
            streamer_id: user_id,
            streamer_login: user_login,
            streamer_name: user_name,
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
