// Configuration structs

use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::{fs, path::PathBuf};

use crate::util;

pub fn from_vodbot_dir(dirs: &[&str]) -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    log::trace!("config dir: {}", path.to_str().unwrap());
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
    let file = fs::File::open(path).map_err(|why| util::ExitMsg::new(
        util::ExitCode::CannotOpenConfig,
        format!(
            "Failed to open config at `{}`, reason: \"{}\".",
            &path.display(),
            why
        ),
    ))?;

    let json: Config = serde_json::from_reader(file).map_err(|why| util::ExitMsg::new(
        util::ExitCode::CannotParseConfig,
        format!(
            "Failed to parse config at `{}`, reason: \"{}\".",
            &path.display(),
            why
        ),
    ))?;

    json.validate().map_err(|why| util::ExitMsg::new(
        util::ExitCode::CannotValidateConfig,
        format!(
            "Failed to validate config at `{}`, reason: \"{}\".",
            &path.display(),
            why
        ),
    ))?;

    Ok(json)
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChatExportFormat {
    Raw,  // JSON export
    Ytt,  // YouTube Timed Text
    Rt,   // RealText
    Sami, // Synchronized Accessible Media Interchange
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum YTTAlignment {
    Left,
    Right,
    Center,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

structstruck::strike! {
    #[strikethrough[derive(Debug, Serialize, Deserialize, Validate)]]
    #[strikethrough[serde(default, rename_all = "camelCase")]]
    pub struct Config {
        #[validate]
        pub channels: Vec<pub struct ConfigChannel {
            #[validate(min_length = 3)]
            #[validate(max_length = 24)]
            pub username: String,
            pub save_vods: bool,
            pub save_highlights: bool,
            pub save_uploads: bool,
            pub save_premieres: bool,
            pub save_clips: bool,
            pub save_chat: bool,
        }>,
        #[validate]
        pub pull: pub struct ConfigPull {
            pub save_vods: bool,
            pub save_highlights: bool,
            pub save_uploads: bool,
            pub save_premieres: bool,
            pub save_clips: bool,
            pub save_chat: bool,

            pub gql_client_id: String,

            pub download_workers: usize,
            pub connection_retries: usize,
            pub connection_timeout: usize,
        },
        #[validate]
        pub chat: pub struct ConfigChat {
            pub export_format: ChatExportFormat,
            pub message_display_time: usize,
            pub randomize_uncolored_names: bool,

            pub ytt_align: YTTAlignment,
            pub ytt_anchor: YTTAnchor,
            #[validate(minimum = 0)]
            #[validate(maximum = 100)]
            pub ytt_position_x: u8,
            #[validate(minimum = 0)]
            #[validate(maximum = 100)]
            pub ytt_position_y: u8,
        },
        #[validate]
        pub stage: pub struct ConfigStage {
            #[validate(pattern = r"^[+-]\d{4}$")]
            pub timezone: String,
            pub description_macros: Vec<String>,
            pub delete_on_export: bool,
            pub delete_on_upload: bool,
        },
        #[validate]
        pub export: pub struct ConfigExport {
            pub ffmpeg_loglevel: FFMPEGLogLevel,
            pub ffmpeg_stderr: Option<PathBuf>,
            pub video_enable: bool,
            pub chat_enable: bool,
            pub thumbnail_enable: bool,
        },
        #[validate]
        pub upload: pub struct ConfigUpload {
            pub chat_enable: bool,
            pub thumbnail_enable: bool,
            pub client_url: String,
            pub client_path: PathBuf,
            pub session_path: PathBuf,
            #[validate(minimum = 262144)]
            pub chunk_size: usize,
            pub oauth_port: u16,
            pub notify_subscribers: bool,
        },
        // #[validate]
        // pub webhooks: ConfigWebhooks,
        // #[validate]
        // pub thumbnail: ConfigThumbnail,
        #[validate]
        pub directories: pub struct ConfigDirectories {
            pub vods: PathBuf,
            pub chat: PathBuf,
            pub highlights: PathBuf,
            pub uploads: PathBuf,
            pub premieres: PathBuf,
            pub clips: PathBuf,

            pub temp: PathBuf,
            pub stage: PathBuf,
            pub thumbnail: PathBuf,
        },
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            channels: Vec::new(),
            pull: ConfigPull::default(),
            chat: ConfigChat::default(),
            stage: ConfigStage::default(),
            export: ConfigExport::default(),
            upload: ConfigUpload::default(),
            directories: ConfigDirectories::default(),
        }
    }
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
            connection_retries: 5,
            connection_timeout: 10,
        }
    }
}
impl Default for ConfigChat {
    fn default() -> Self {
        Self {
            export_format: ChatExportFormat::Ytt,
            message_display_time: 10,
            randomize_uncolored_names: true,

            ytt_align: YTTAlignment::Left,
            ytt_anchor: YTTAnchor::BottomLeft,
            ytt_position_x: 0,
            ytt_position_y: 100,
        }
    }
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
impl Default for ConfigDirectories {
    fn default() -> Self {
        Self {
            vods: from_vodbot_dir(&["videos", "vods"]),
            chat: from_vodbot_dir(&["videos", "chat"]),
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
