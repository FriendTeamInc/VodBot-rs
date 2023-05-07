// Configuration structs

use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct ConfigChannel {
    #[validate(min_length = 3)]
    #[validate(max_length = 24)]
    pub username: String,

    pub save_vods: bool,
    pub save_highlights: bool,
    pub save_uploads: bool,
    pub save_premieres: bool,
    pub save_clips: bool,
    pub save_chat: bool,
}
impl Default for ConfigChannel {
    fn default() -> Self {
        Self {
            username: String::from(""),
            save_vods: true,
            save_highlights: true,
            save_uploads: true,
            save_premieres: true,
            save_clips: true,
            save_chat: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct ConfigPull {
    pub save_vods: bool,
    pub save_highlights: bool,
    pub save_uploads: bool,
    pub save_premieres: bool,
    pub save_clips: bool,
    pub save_chat: bool,

    pub gql_client_id: String,

    pub max_download_workers: usize,
    pub download_chunk_size: usize,

    pub connection_retries: usize,
    pub connection_timeout: usize,
}
impl Default for ConfigPull {
    fn default() -> Self {
        Self {
            save_vods: true,
            save_highlights: true,
            save_uploads: true,
            save_premieres: true,
            save_clips: true,
            save_chat: true,

            gql_client_id: String::from("kd1unb4b3q4t58fwlpcbzcbnm76a8fp"),
            max_download_workers: num_cpus::get(),
            download_chunk_size: 1024,
            connection_retries: 5,
            connection_timeout: 5,
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, Validate)]
// pub struct ConfigWebhookBase {

// }

// #[derive(Debug, Serialize, Deserialize, Validate)]
// pub struct ConfigWebhooks {

// }

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct ConfigDirectories {
    pub vods: PathBuf,
    pub highlights: PathBuf,
    pub uploads: PathBuf,
    pub premieres: PathBuf,
    pub clips: PathBuf,

    pub temp: PathBuf,
    pub stage: PathBuf,
    pub thumbnail: PathBuf,
}
impl Default for ConfigDirectories {
    fn default() -> Self {
        Self {
            vods: from_vodbot_dir(&["videos", "vods"]),
            highlights: from_vodbot_dir(&["videos", "highlights"]),
            uploads: from_vodbot_dir(&["videos", "uploads"]),
            premieres: from_vodbot_dir(&["videos", "premieres"]),
            clips: from_vodbot_dir(&["videos", "clips"]),
            temp: from_vodbot_dir(&["temp"]),
            stage: from_vodbot_dir(&["stage"]),
            thumbnail: from_vodbot_dir(&["thumbnail"]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct Config {
    pub channels: Vec<ConfigChannel>,
    pub pull: ConfigPull,
    pub directories: ConfigDirectories,
}

fn from_vodbot_dir(dirs: &[&str]) -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("vodbot");
    for dir in dirs {
        path.push(dir);
    }
    path
}
