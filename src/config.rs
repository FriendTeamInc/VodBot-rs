// Configuration structs

use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ConfigChannel {
    #[validate(min_length = 3)]
    #[validate(max_length = 24)]
    username: String,

    #[serde(default = "_default_true")]
    save_vods: bool,
    #[serde(default = "_default_true")]
    save_highlights: bool,
    #[serde(default = "_default_true")]
    save_uploads: bool,
    #[serde(default = "_default_true")]
    save_premieres: bool,
    #[serde(default = "_default_true")]
    save_clips: bool,
    #[serde(default = "_default_true")]
    save_chat: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ConfigPull {
    #[serde(default = "_default_true")]
    save_vods: bool,
    #[serde(default = "_default_true")]
    save_highlights: bool,
    #[serde(default = "_default_true")]
    save_uploads: bool,
    #[serde(default = "_default_true")]
    save_premieres: bool,
    #[serde(default = "_default_true")]
    save_clips: bool,
    #[serde(default = "_default_true")]
    save_chat: bool,

    #[serde(default = "_default_twitch_client_id")]
    gql_client_id: String,

    #[serde(default = "num_cpus::get")]
    max_download_workers: usize,
    #[serde(default = "_default_download_chunk_size")]
    download_chunk_size: usize,

    #[serde(default = "_default_connection_retries")]
    connection_retries: usize,
    #[serde(default = "_default_connection_timeout")]
    connection_timeout: usize,
}

// #[derive(Debug, Serialize, Deserialize, Validate)]
// struct ConfigWebhookBase {

// }

// #[derive(Debug, Serialize, Deserialize, Validate)]
// struct ConfigWebhooks {

// }

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ConfigDirectories {
    #[serde(default = "_default_vods_directory")]
    vods: PathBuf,
    #[serde(default = "_default_highlights_directory")]
    highlights: PathBuf,
    #[serde(default = "_default_uploads_directory")]
    uploads: PathBuf,
    #[serde(default = "_default_premieres_directory")]
    premieres: PathBuf,
    #[serde(default = "_default_clips_directory")]
    clips: PathBuf,

    #[serde(default = "_default_temp_directory")]
    temp: PathBuf,
    #[serde(default = "_default_stage_directory")]
    stage: PathBuf,
    #[serde(default = "_default_thumbnail_directory")]
    thumbnail: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct Config {
    channels: Vec<ConfigChannel>,
    pull: ConfigPull,
    directories: ConfigDirectories,
}

const fn _default_true() -> bool {
    true
}

fn _default_twitch_client_id() -> String {
    String::from("kd1unb4b3q4t58fwlpcbzcbnm76a8fp")
}

const fn _default_download_chunk_size() -> usize {
    1024
}

const fn _default_connection_retries() -> usize {
    5
}

const fn _default_connection_timeout() -> usize {
    5
}

fn _default_vodbot_directory() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("vodbot");
    path
}

fn _default_vods_directory() -> PathBuf {
    let mut path = _default_vodbot_directory();
    path.push("videos");
    path.push("vods");
    path
}

fn _default_highlights_directory() -> PathBuf {
    let mut path = _default_vodbot_directory();
    path.push("videos");
    path.push("highlights");
    path
}

fn _default_uploads_directory() -> PathBuf {
    let mut path = _default_vodbot_directory();
    path.push("videos");
    path.push("uploads");
    path
}

fn _default_premieres_directory() -> PathBuf {
    let mut path = _default_vodbot_directory();
    path.push("videos");
    path.push("premieres");
    path
}

fn _default_clips_directory() -> PathBuf {
    let mut path = _default_vodbot_directory();
    path.push("videos");
    path.push("clips");
    path
}

fn _default_temp_directory() -> PathBuf {
    let mut path = _default_vodbot_directory();
    path.push("temp");
    path
}

fn _default_stage_directory() -> PathBuf {
    let mut path = _default_vodbot_directory();
    path.push("stage");
    path
}

fn _default_thumbnail_directory() -> PathBuf {
    let mut path = _default_vodbot_directory();
    path.push("thumbnail");
    path
}
