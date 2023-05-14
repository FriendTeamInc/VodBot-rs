// Info command, for getting basic data on various things

use crate::config::load_config;
use crate::gql::GQLClient;
use crate::twitch;
use crate::util::ExitMsg;

use regex::Regex;
use std::path::PathBuf;

#[derive(Debug)]
enum ContentType {
    Channel,
    Video,
    Clip,
}

#[rustfmt::skip]
fn test(client: &GQLClient) -> Result<(), ExitMsg> {
    println!("channel_videos:    \n{:?}", twitch::get_channel_videos(&client, "vodbot_fti".to_owned())?);
    println!("channels_videos:   \n{:?}", twitch::get_channels_videos(&client, vec!["vodbot_fti".to_owned()])?);
    println!("video_chapters:    \n{:?}", twitch::get_video_chapters(&client, "1818343419".to_owned())?);
    println!("videos_chapters:   \n{:?}", twitch::get_videos_chapters(&client, vec!["1818343419".to_owned()])?);
    println!("video_comments:    \n{:?}", twitch::get_video_comments(&client, "1818343419".to_owned())?);
    println!("videos_comments:   \n{:?}", twitch::get_videos_comments(&client, vec!["1818343419".to_owned()])?);
    println!("video_pba_token:   \n{:?}", twitch::get_video_playback_access_token(&client, "1818343419".to_owned())?);
    println!("videos_pba_tokens: \n{:?}", twitch::get_videos_playback_access_tokens(&client, vec!["1811624369".to_owned()])?);
    // println!("                   \n");
    println!("channel_clips:     \n{:?}", twitch::get_channel_clips(&client, "vodbot_fti".to_owned())?);
    println!("channels_clips:    \n{:?}", twitch::get_channels_clips(&client, vec!["vodbot_fti".to_owned()])?);
    println!("clip_pba_token:    \n{:?}", twitch::get_clip_playback_access_token(&client, "SourHardLEDDBstyle-NMdErh41r1IN9cjm".to_owned())?);
    println!("clip_pba_tokens:   \n{:?}", twitch::get_clips_playback_access_tokens(&client, vec!["SourHardLEDDBstyle-NMdErh41r1IN9cjm".to_owned()])?);
    // println!("                   \n");
    println!("get_channel:       \n{:?}", twitch::get_channel(&client, "vodbot_fti".to_owned())?);
    println!("get_video:         \n{:?}", twitch::get_video(&client, "1818343419".to_owned())?);
    println!("get_clip:          \n{:?}", twitch::get_clip(&client, "SourHardLEDDBstyle-NMdErh41r1IN9cjm".to_owned())?);
    println!("                   \n\n\n\n");

    Ok(())
}

pub fn run(config_path: PathBuf, _json: bool, ids: Vec<String>) -> Result<(), ExitMsg> {
    let set = [
        (ContentType::Video, Regex::new(r"^(?P<id>\d+)?$").unwrap()),
        (ContentType::Video, Regex::new(r"^(https?://)?(www\.)?twitch.tv/videos/(?P<id>\d+)(\?.*)?$").unwrap()),
        (ContentType::Channel, Regex::new(r"^(?P<id>[a-zA-Z0-9][\w]{3,24})$").unwrap()),
        (ContentType::Channel, Regex::new(r"^(https?://)?(www\.)?twitch\.tv/(?P<id>[a-zA-Z0-9][\w]{3,24})(\?.*)?$").unwrap()),
        (ContentType::Clip, Regex::new(r"^(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)$").unwrap()),
        (ContentType::Clip, Regex::new(r"^(https?://)?(www\.)?twitch.tv/\w+/clip/(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)(\?.*)?$").unwrap()),
        (ContentType::Clip, Regex::new(r"^(https?://)?clips\.twitch.tv/(?P<id>[A-Za-z0-9]+(?:-[A-Za-z0-9_-]{16})?)(\?.*)?$").unwrap()),
    ];

    let conf = load_config(&config_path)?;

    // Let's map all the arguments given by the user to a regex (to extract an
    // ID from) and type to know what to query.
    let s = ids.iter().filter_map(|id| {
        set.iter().find_map(|(kind, rgx)| {
            rgx.captures(id)
                .and_then(|c| c.name("id"))
                .map(|c| (kind, c.as_str()))
        })
    });

    let client = GQLClient::new(conf.pull.gql_client_id);
    test(&client)?;

    for i in s {
        // we know the type and the id now, we can make queries here (or in a map)
        let j = i.1.to_owned();
        match i.0 {
            ContentType::Channel => {
                let r = twitch::get_channel(&client, j)?;
                println!("get_channel: \n{:?}\n", r);
            }
            ContentType::Video => {
                let r = twitch::get_video(&client, j.clone())?;
                if let None = r {
                    let r = twitch::get_channel(&client, j)?;
                    println!("get_channel (after video): \n{:?}\n", r);
                } else {
                    println!("get_video: \n{:?}\n", r);
                }
            }
            ContentType::Clip => {
                let r = twitch::get_clip(&client, j)?;
                println!("get_clip: \n{:?}\n", r);
            }
        }
    }

    Ok(())
}
