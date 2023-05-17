// Pull command, for grabbing videos off of Twitch

use crate::cli::PullMode;
use crate::config::{load_config, ConfigChannel};
use crate::gql::GQLClient;
use crate::itd;
use crate::twitch;
use crate::util::ExitMsg;

use std::collections::HashMap;
use std::path::PathBuf;

pub fn run(config_path: PathBuf, mode: PullMode) -> Result<(), ExitMsg> {
    let conf = load_config(&config_path)?;
    let c = &conf.channels;

    let f = |f: &ConfigChannel| f.username.clone();
    let users: Vec<_> = c.iter().map(f).collect();
    println!("Checking users: {} ...", users.join(", "));

    let save_vods = conf.pull.save_vods;
    // let save_chat = conf.pull.save_chat;
    let save_clips = conf.pull.save_clips;
    // let save_highlights = conf.pull.save_highlights;
    // let save_premieres = conf.pull.save_premieres;
    // let save_uploads = conf.pull.save_uploads;

    let vods: Vec<_> = c
        .iter()
        .filter(|f| f.save_vods && save_vods)
        .map(f)
        .collect();
    // let chat: Vec<_> = c.iter().filter(|f| f.save_chat && save_chat).map(f).collect();
    let clips: Vec<_> = c
        .iter()
        .filter(|f| f.save_clips && save_clips)
        .map(f)
        .collect();
    // let highlights: Vec<_> = c.iter().filter(|f| f.save_highlights && save_highlights).map(f).collect();
    // let premieres: Vec<_> = c.iter().filter(|f| f.save_premieres && save_premieres).map(f).collect();
    // let uploads: Vec<_> = c.iter().filter(|f| f.save_uploads && save_uploads).map(f).collect();

    let client = GQLClient::new(conf.pull.gql_client_id.clone());

    let vods = twitch::get_channels_videos(&client, vods)?;
    // let chat = twitch::get_channels_videos(&client, chat); // gotta filter and map with vods
    let clips = twitch::get_channels_clips(&client, clips)?;

    // TODO: check disk and filter out existing videos
    // make a hashmap of usernames and video ids for vods, clips, etc
    // then do a filter on the above hashmaps

    let vods_count: HashMap<_, _> = vods.iter().map(|(k, v)| (k, v.len())).collect();
    let vods_total: usize = vods_count.values().into_iter().sum();
    // let chat_count: HashMap<_, _> = chat.iter().map(|(k, v)| (k, v.len())).collect();
    let clips_count: HashMap<_, _> = clips.iter().map(|(k, v)| (k, v.len())).collect();
    let clips_total: usize = clips_count.values().into_iter().sum();

    let counts: HashMap<_, _> = users
        .iter()
        .map(|f| {
            (
                f.to_owned(),
                (
                    vods_count.get(f).unwrap_or(&0),
                    // chat_count.get(f).unwrap_or(&0),
                    clips_count.get(f).unwrap_or(&0),
                ),
            )
        })
        .collect();

    // we go by the order in the config, not whatever arbitrary order the hashmap may give
    for k in &users {
        let v = counts.get(k).unwrap();
        println!("{}: {} Vods & {} Clips", k, v.0, v.1);
    }
    println!("Total Vods: {}", vods_total);
    println!("Total Clips: {}", clips_total);
    println!("");

    // create a new client (for making generic http requests)
    let genclient = reqwest::blocking::Client::new();

    // now to download each set of videos per user
    for k in &users {
        println!("Downloading videos for {} ...", k);

        let voddir = conf.directories.vods.clone();
        let vods = vods.get(k).unwrap().to_owned();
        let tokens = twitch::get_videos_playback_access_tokens(
            &client,
            vods.iter().map(|f| f.id.to_owned()).collect(),
        )?;

        let vod_tup: Vec<_> = vods
            .iter()
            .map(|f| {
                (
                    f.to_owned(),
                    tokens.get(&f.id).unwrap().to_owned(),
                    voddir
                        .clone()
                        .join(format!("{}_{}.mkv", f.created_at.replace(":", ";"), f.id)),
                )
            })
            .collect();

        itd::download_vods(&conf, vod_tup, &genclient)?;
        // TODO: check through vods and write meta files as necessary

        let clipdir = conf.directories.clips.clone();
        let clips = clips.get(k).unwrap().to_owned();
        let tokens = twitch::get_clips_playback_access_tokens(
            &client,
            clips.iter().map(|f| f.slug.to_owned()).collect(),
        )?;

        let clip_tup: Vec<_> = clips
            .iter()
            .map(|f| {
                (
                    f.to_owned(),
                    tokens.get(&f.slug).unwrap().to_owned(),
                    clipdir
                        .clone()
                        .join(format!("{}_{}.mkv", f.created_at.replace(":", ";"), f.slug)),
                )
            })
            .collect();

        itd::download_clips(&conf, clip_tup, &genclient)?;

        println!("");
    }

    Ok(())
}
