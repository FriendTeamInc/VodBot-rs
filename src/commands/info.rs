// Info command, for getting basic data on various things

use crate::config;
use crate::gql;
use crate::twitch;
use crate::util;

use indoc::printdoc;
use regex::Regex;
use std::path::PathBuf;

#[derive(Debug)]
enum ContentType {
    Channel,
    Video,
    Clip,
}

pub fn run(config_path: PathBuf, _json: bool, ids: Vec<String>) -> Result<(), util::ExitMsg> {
    let set = [
        (ContentType::Channel, Regex::new(r"^(?P<id>[a-zA-Z0-9_][\w]{4,25})$").unwrap()),
        (ContentType::Channel, Regex::new(r"^(https?://)?(www\.)?twitch\.tv/(?P<id>[a-zA-Z0-9][\w]{4,25})(\?.*)?$").unwrap()),
        (ContentType::Video, Regex::new(r"^(?P<id>\d+)?$").unwrap()),
        (ContentType::Video, Regex::new(r"^(https?://)?(www\.)?twitch.tv/videos/(?P<id>\d+)(\?.*)?$").unwrap()),
        (ContentType::Clip, Regex::new(r"^(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)$").unwrap()),
        (ContentType::Clip, Regex::new(r"^(https?://)?(www\.)?twitch.tv/\w+/clip/(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)(\?.*)?$").unwrap()),
        (ContentType::Clip, Regex::new(r"^(https?://)?clips\.twitch.tv/(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)(\?.*)?$").unwrap()),
    ];

    let conf = config::load_config(&config_path)?;

    // Let's map all the arguments given by the user to a regex (to extract an
    // ID from) and type to know what to query.
    let s = ids.iter().filter_map(|id| {
        set.iter().find_map(|(kind, rgx)| {
            rgx.captures(id)
                .and_then(|c| c.name("id"))
                .map(|c| (kind, c.as_str()))
        })
    });

    for i in s {
        // we know the type and the id now, we can make queries here (or in a map)
        println!("{:?} {:?}", i.0, i.1);
    }

    let client = gql::GQLClient::new(conf.pull.gql_client_id);
    printdoc!(
        "
        get_channel_videos:               \n{:?}\n
        get_channels_videos:              \n{:?}\n
        get_video_chapters:               \n{:?}\n
        get_videos_chapters:              \n{:?}\n
        get_video_comments:               \n{:?}\n
        get_videos_comments:              \n{:?}\n
        get_video_playback_access_token:  \n{:?}\n
        get_video_playback_access_tokens: \n{:?}\n
        \n\n
        get_channel_clips:                \n{:?}\n
        get_channels_clips:               \n{:?}\n
        get_clip_playback_access_token:   \n{:?}\n
        get_clip_playback_access_tokens:  \n{:?}\n
        \n",
        twitch::get_channel_videos(&client, "vodbot_fti".to_owned())?,
        twitch::get_channels_videos(&client, vec!["vodbot_fti".to_owned()])?,
        twitch::get_video_chapters(&client, "1818343419".to_owned())?,
        twitch::get_videos_chapters(&client, vec!["1818343419".to_owned()])?,
        twitch::get_video_comments(&client, "1818343419".to_owned())?,
        twitch::get_videos_comments(&client, vec!["1818343419".to_owned()])?,
        twitch::get_video_playback_access_token(&client, "1818343419".to_owned())?,
        twitch::get_videos_playback_access_tokens(&client, vec!["1811624369".to_owned()])?,
        twitch::get_channel_clips(&client, "vodbot_fti".to_owned())?,
        twitch::get_channels_clips(&client, vec!["vodbot_fti".to_owned()])?,
        twitch::get_clip_playback_access_token(
            &client,
            "SourHardLEDDBstyle-NMdErh41r1IN9cjm".to_owned()
        )?,
        twitch::get_clips_playback_access_tokens(
            &client,
            vec!["SourHardLEDDBstyle-NMdErh41r1IN9cjm".to_owned()]
        )?,
    );
    
    println!("get_channel: {:?}", twitch::get_channel(&client, "vodbot_fti".to_owned())?);
    println!("get_video:   {:?}", twitch::get_video(&client, "1818343419".to_owned())?);
    println!("get_clip:    {:?}", twitch::get_clip(&client, "SourHardLEDDBstyle-NMdErh41r1IN9cjm".to_owned())?);

    Ok(())
}
