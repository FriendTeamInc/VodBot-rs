// Utility functions and types

use std::fs;
use std::path::Path;

// #[derive(Copy, Clone)]
pub enum ExitCode {
    _CleanExit,
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
