// Configuration structs

use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::{fs, path::PathBuf};

use crate::util;

pub fn from_vodbot_dir(dirs: &[&str]) -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("vodbot");
    for dir in dirs {
        path.push(dir);
    }
    path
}

pub fn default_config_location() -> PathBuf {
    from_vodbot_dir(&["config.json"])
}

pub fn load_config(path: &PathBuf) -> Result<Config, util::ExitMsg> {
    let file = fs::File::open(path).map_err(|why| util::ExitMsg {
        code: util::ExitCode::CannotOpenConfig,
        msg: format!(
            "Failed to open config at `{}`, reason: \"{}\".",
            &path.display(),
            why
        ),
    })?;
    let json: Config = serde_json::from_reader(file).map_err(|why| util::ExitMsg {
        code: util::ExitCode::CannotParseConfig,
        msg: format!(
            "Failed to parse config at `{}`, reason: \"{}\".",
            &path.display(),
            why
        ),
    })?;

    json.validate().map_err(|why| util::ExitMsg {
        code: util::ExitCode::CannotValidateConfig,
        msg: format!(
            "Failed to validate config at `{}`, reason: \"{}\".",
            &path.display(),
            why
        ),
    })?;

    Ok(json)
}

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

    pub download_workers: usize,
    // pub download_chunk_size: usize,
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
            download_workers: num_cpus::get(),
            // download_chunk_size: 1024,
            connection_retries: 5,
            connection_timeout: 10,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExportFormatType {
    Raw,  // JSON export
    YTT,  // YouTube Timed Text
    RT,   // RealText
    SAMI, // Synchronized Accessible Media Interchange
}

#[derive(Debug, Serialize, Deserialize)]
pub enum YTTAlignment {
    Left,
    Right,
    Center,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum YTTAnchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    CenterCenter,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct ConfigChat {
    pub export_format: ExportFormatType, // TODO: change this to enum
    pub message_display_time: usize,
    pub randomize_uncolored_names: bool,

    pub ytt_align: YTTAlignment, // TODO: change this to enum
    pub ytt_anchor: YTTAnchor,
    #[validate(minimum = 0)]
    #[validate(maximum = 100)]
    pub ytt_position_x: u8,
    #[validate(minimum = 0)]
    #[validate(maximum = 100)]
    pub ytt_position_y: u8,
}
impl Default for ConfigChat {
    fn default() -> Self {
        Self {
            export_format: ExportFormatType::YTT,
            message_display_time: 10,
            randomize_uncolored_names: true,

            ytt_align: YTTAlignment::Left,
            ytt_anchor: YTTAnchor::BottomLeft,
            ytt_position_x: 0,
            ytt_position_y: 100,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct ConfigStage {
    #[validate(pattern = r"^[+-]\d{4}$")]
    timezone: String,
    description_macros: Vec<String>,
    delete_on_export: bool,
    delete_on_upload: bool,
}
impl Default for ConfigStage {
    fn default() -> Self {
        Self {
            timezone: String::from("+0000"),
            description_macros: Vec::new(),
            delete_on_export: false,
            delete_on_upload: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FFMPEGLogLevel {
    Quiet,
    Panic,
    Fatal,
    Error,
    Warning,
    Info,
    Verbose,
    Debug,
    Trace,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct ConfigExport {
    ffmpeg_loglevel: FFMPEGLogLevel,
    ffmpeg_stderr: Option<PathBuf>,
    video_enable: bool,
    chat_enable: bool,
    thumbnail_enable: bool,
}
impl Default for ConfigExport {
    fn default() -> Self {
        Self {
            ffmpeg_loglevel: FFMPEGLogLevel::Warning,
            ffmpeg_stderr: None,
            video_enable: true,
            chat_enable: true,
            thumbnail_enable: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(default)]
pub struct ConfigUpload {
    chat_enable: bool,
    thumbnail_enable: bool,
    client_url: String,
    client_path: PathBuf,
    session_path: PathBuf,
    #[validate(minimum = 262144)]
    chunk_size: usize,
    oauth_port: u16,
    notify_subscribers: bool,
}
impl Default for ConfigUpload {
    fn default() -> Self {
        Self {
            chat_enable: true,
            thumbnail_enable: true,
            client_url: String::from(
                "https://www.friendteam.biz/assets/vodbot-youtube-credentials",
            ),
            client_path: from_vodbot_dir(&["youtube_client.json"]),
            session_path: from_vodbot_dir(&["youtube_session.json"]),
            chunk_size: 262144,
            oauth_port: 8080,
            notify_subscribers: true,
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, Validate)]
// #[serde(default)]
// pub struct ConfigThumbnailIcon {  }
// #[derive(Debug, Serialize, Deserialize, Validate)]
// #[serde(default)]
// pub struct ConfigThumbnailPosition {  }
// #[derive(Debug, Serialize, Deserialize, Validate)]
// #[serde(default)]
// pub struct ConfigThumbnail {  }

// #[derive(Debug, Serialize, Deserialize, Validate)]
// #[serde(default)]
// pub struct ConfigWebhookBase {  }
// #[derive(Debug, Serialize, Deserialize, Validate)]
// #[serde(default)]
// pub struct ConfigWebhooks {  }

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
    #[validate]
    pub channels: Vec<ConfigChannel>,
    #[validate]
    pub pull: ConfigPull,
    #[validate]
    pub chat: ConfigChat,
    #[validate]
    pub stage: ConfigStage,
    #[validate]
    pub export: ConfigExport,
    #[validate]
    pub upload: ConfigUpload,
    // #[validate]
    // pub webhooks: ConfigWebhooks,
    // #[validate]
    // pub thumbnail: ConfigThumbnail,
    #[validate]
    pub directories: ConfigDirectories,
}
