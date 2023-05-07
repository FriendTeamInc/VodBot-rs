// Utility functions and types

use std::fs;
use std::path::Path;

// Self-describing exit codes.
// Each exit point of the program should be using a very clear exit code, along
// with a message sent to stderr for more details. Certain codes are reserved or
// not used, as indicated by the leading underscore in its name.
// #[derive(Copy, Clone)]
pub enum ExitCode {
    _CleanExit,
    UnknownError,
    _ReservedByClap,
    CannotCreateDir,
}

pub struct ExitMsg {
    pub code: ExitCode,
    pub msg: String,
}

pub fn create_dir(dir_path: &Path) -> Result<(), ExitMsg> {
    fs::create_dir_all(&dir_path).map_err(|why| ExitMsg {
        code: ExitCode::CannotCreateDir,
        msg: format!(
            "Cannot create directory `{}`, reason \"{}\".",
            &dir_path.display(),
            why
        ),
    })
}
