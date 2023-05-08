// Info command, for getting basic data on various things

use crate::config;
use crate::util;

enum PatternType {
    Channel,
    Video,
    Clip,
}

// Regex patterns for detecting channels, videos, and clips
const PATTERNS: &'static [[&str; 3]; 3] = &[
    [
        r"^(?P<id>[a-zA-Z0-9][\w]{0,24})$",
        r"^(https?://)?(www\.)?twitch\.tv/(?P<id>[a-zA-Z0-9][\w]{0,24})(\?.*)?$",
        r"",
    ],
    [
        r"^(?P<id>\d+)?$",
        r"^(https?://)?(www\.)?twitch.tv/videos/(?P<id>\d+)(\?.*)?$",
        r"",
    ],
    [
        r"^(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)$",
        r"^(https?://)?(www\.)?twitch.tv/\w+/clip/(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)(\?.*)?$",
        r"^(https?://)?clips\.twitch.tv/(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)(\?.*)?$",
    ],
];

pub fn run(json: bool, strings: Vec<String>) -> Result<(), util::ExitMsg> {
    let config_path = config::default_config_location();
    let conf = config::load_config(&config_path)?;
    println!("{:#?}", conf);

    loop {}

    Ok(())
}
