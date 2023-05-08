// Info command, for getting basic data on various things

use crate::config;
use crate::util;

use regex::Regex;

#[derive(Debug)]
enum ContentType {
    Channel,
    Video,
    Clip
}

pub fn run(json: bool, strings: Vec<String>) -> Result<(), util::ExitMsg> {
    let set = [
        (ContentType::Channel, Regex::new(r"^(?P<id>[a-zA-Z0-9_][\w]{4,25})$").unwrap()),
        (ContentType::Channel, Regex::new(r"^(https?://)?(www\.)?twitch\.tv/(?P<id>[a-zA-Z0-9][\w]{4,25})(\?.*)?$").unwrap()),
        (ContentType::Video, Regex::new(r"^(?P<id>\d+)?$").unwrap()),
        (ContentType::Video, Regex::new(r"^(https?://)?(www\.)?twitch.tv/videos/(?P<id>\d+)(\?.*)?$").unwrap()),
        (ContentType::Clip, Regex::new(r"^(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)$").unwrap()),
        (ContentType::Clip, Regex::new(r"^(https?://)?(www\.)?twitch.tv/\w+/clip/(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)(\?.*)?$").unwrap()),
        (ContentType::Clip, Regex::new(r"^(https?://)?clips\.twitch.tv/(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)(\?.*)?$").unwrap()),
    ];

    let config_path = config::default_config_location();
    let conf = config::load_config(&config_path)?;

    let s = strings.iter().filter_map(|id| {
        set.iter().filter_map(|r| if r.1.captures(id).is_some() {Some(r)} else {None}).next()
    });

    for i in s {
        // we know the type and the id now, we can make queries here (or in a map)
        println!("{:?} {:?}", i.0, i.1);
    }

    Ok(())
}
