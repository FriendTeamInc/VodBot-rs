// Structs for handling VodBot specifically generated data

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Pull related data

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VodChapter {
    pub position: String,
    pub duration: String,
    // pub r#type: VodChapterType,
    pub filepath: PathBuf,
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
