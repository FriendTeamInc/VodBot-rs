// Utility functions and types

use std::fs;
use std::path::Path;

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

    // Generic codes
    CannotRegisterSignalHandler,
    CannotCreateDir,

    CannotOpenConfig,
    CannotParseConfig,
    CannotValidateConfig,

    CannotConnectToTwitch,
    RequestErrorFromTwitch, // TODO: rename this one
    GQLErrorFromTwitch,
    CannotParseResponseFromTwitch,

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
    PullCannotCreateThreadPool,
    PullCannotGetChunk,
    PullCannotParseChunk,
    PullCannotWriteChunk,
}

#[derive(Debug, Clone)]
pub struct ExitMsg {
    pub code: ExitCode,
    pub msg: String,
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
    fs::create_dir_all(&dir_path).map_err(|why| ExitMsg {
        code: ExitCode::CannotCreateDir,
        msg: format!(
            "Cannot create directory `{}`, reason: \"{}\".",
            &dir_path.display(),
            why
        ),
    })
}
