// Utility functions and types

use std::fs;
use std::path::{Path, PathBuf};

// Self-describing exit codes.
// Each exit point of the program should be using a very clear exit code, along
// with a message sent to stderr for more details. Certain codes may be reserved
// or not used, as indicated by the leading underscore in its name.
#[derive(Debug, Clone)]
pub enum ExitCode {
    // Special codes
    _CleanExit,
    Interrupted,
    _ReservedByClap,
    StderrLoggerError,

    // Generic codes
    CannotRegisterSignalHandler,
    CannotCreateDir,
    CannotChangeDir,

    CannotOpenConfig,
    CannotParseConfig,
    CannotValidateConfig,

    CannotConnectToTwitch,
    RequestErrorFromTwitch, // TODO: rename this one
    GQLErrorFromTwitch,
    CannotParseResponseFromTwitch,

    CannotStartFfmpeg,
    FfmpegReturnedError,
    FfmpegInterrupted,

    CannotGlobDirectory,

    // Command-specific codes
    InitCannotOpenConfig,
    InitCannotWriteConfig,
    InitCannotSerializeConfig,

    PullCannotGetPlaylistURI,
    PullCannotReadPlaylistURI,
    PullCannotParsePlaylistURI,
    PullCannotFindPlaylistURI,
    PullCannotGetSourcePlaylist,
    PullCannotReadSourcePlaylist,
    PullCannotParseSourcePlaylist,
    PullCannotUseSourcePlaylist,
    PullCannotWriteSourcePlaylist,
    PullCannotCleanUpAfterDownload,
    PullCannotGetChunk,
    PullCannotParseChunk,
    PullCannotWriteChunk,
    PullCannotOpenMeta,
}

#[derive(Debug, Clone)]
pub struct ExitMsg {
    pub code: ExitCode,
    pub msg: String,
}
impl ExitMsg {
    pub fn new(code: ExitCode, msg: String) -> Self {
        log::warn!(
            "ExitMsg - {:?} ({}) - {}",
            code,
            code.clone() as i32,
            msg.as_str()
        );
        ExitMsg {
            code: code,
            msg: msg,
        }
    }
}
impl std::fmt::Display for ExitMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nExit code: {:?} ({})",
            self.msg.as_str(),
            self.code,
            self.code.clone() as i32
        )
    }
}
impl std::error::Error for ExitMsg {}

pub fn create_dir(dir_path: &Path) -> Result<(), ExitMsg> {
    fs::create_dir_all(&dir_path).map_err(|why| {
        ExitMsg::new(
            ExitCode::CannotCreateDir,
            format!(
                "Cannot create directory `{}`, reason: \"{}\".",
                &dir_path.display(),
                why
            ),
        )
    })
}

// replace with the following?
// https://docs.rs/size_format/latest/size_format/
// https://docs.rs/bytesize/latest/bytesize/
pub fn format_size(size: usize, digits: usize, display_units: bool) -> String {
    let mut size = size as f32;
    let units = ["B", "kB", "MB", "GB", "PB", "EB"];
    for u in units {
        if size < 1000.0 {
            let t = format!("{:.1$}", size, digits);
            if display_units {
                return format!("{} {}", t, u);
            } else {
                return format!("{}", t);
            }
        }

        size /= 1000.0;
    }

    let t = format!("{:.1$}", size, digits);
    if display_units {
        return format!("{} ZB", t);
    } else {
        return format!("{}", t);
    }
}

pub fn chdir(path: &PathBuf) -> Result<(), ExitMsg> {
    log::debug!("changing directory to {}", path.to_str().unwrap());
    std::env::set_current_dir(path).map_err(|why| {
        ExitMsg::new(
            ExitCode::CannotChangeDir,
            format!(
                "Cannot change directory to `{}`, reason: \"{}\".",
                path.to_str().unwrap(),
                why
            ),
        )
    })?;

    Ok(())
}

pub fn get_meta_ids(path: PathBuf) -> Result<Vec<String>, ExitMsg> {
    let path = path.join("*.meta.json");
    let path = path.to_str().unwrap();

    // TODO: remove glob and just list_dir manually

    Ok(glob::glob(path)
        .map_err(|why| {
            ExitMsg::new(
                ExitCode::CannotGlobDirectory,
                format!("Failed to glob/wildcard directory, reason `{}`.", why),
            )
        })?
        .filter_map(|f| f.ok())
        .map(|f| {
            let s = f.file_name().unwrap().to_str().unwrap();
            String::from(&s[21..(s.len() - 10)])
        })
        .collect())
}
