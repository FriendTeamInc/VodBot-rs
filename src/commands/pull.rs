// Pull command, for grabbing videos off of Twitch

use crate::cli::PullMode;
use crate::config::{load_config, Config, ConfigChannel};
use crate::gql::GQLClient;
use crate::itd;
use crate::twitch;
use crate::util::{ExitCode, ExitMsg};
use crate::vodbot_api::{Clip, PlaybackAccessToken, Vod, VodBotData};

use reqwest::blocking::Client;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn run(config_path: PathBuf, _mode: PullMode) -> Result<(), ExitMsg> {
    let conf = load_config(&config_path)?;
    let c = &conf.channels;

    let f = |f: &ConfigChannel| f.username.clone();
    let users: Vec<_> = c.iter().map(f).collect();
    println!("Checking users: {} ...", users.join(", "));

    #[rustfmt::skip]
    let s: (Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>) = (
        c.iter().filter(|f| f.save_vods && conf.pull.save_vods).map(f).collect(),
        c.iter().filter(|f| f.save_chat && conf.pull.save_chat).map(f).collect(),
        c.iter().filter(|f| f.save_highlights && conf.pull.save_highlights).map(f).collect(),
        c.iter().filter(|f| f.save_premieres && conf.pull.save_premieres).map(f).collect(),
        c.iter().filter(|f| f.save_uploads && conf.pull.save_uploads).map(f).collect(),
        c.iter().filter(|f| f.save_clips && conf.pull.save_clips).map(f).collect(),
    );

    let client = GQLClient::new(conf.pull.gql_client_id.clone());

    let mut vods = twitch::get_channels_videos_archive(&client, s.0)?;
    // let mut chat = twitch::get_channels_videos(&client, s.1);
    let mut highlights = twitch::get_channels_videos_highlight(&client, s.2)?;
    let mut premieres = twitch::get_channels_videos_premiere(&client, s.3)?;
    let mut uploads = twitch::get_channels_videos_upload(&client, s.4)?;
    let mut clips = twitch::get_channels_clips(&client, s.5)?;

    // TODO: check disk and filter out existing videos
    // make a hashmap of usernames and video ids for vods, clips, etc
    // then do a filter on the above hashmaps

    let vods_count: HashMap<_, _> = vods.iter().map(|(k, v)| (k, v.len())).collect();
    let vods_total: usize = vods_count.values().into_iter().sum();
    // let chat_count: HashMap<_, _> = chat.iter().map(|(k, v)| (k, v.len())).collect();
    // let chat_total: usize = chat_count.values().into_iter().sum();
    let highlights_count: HashMap<_, _> = highlights.iter().map(|(k, v)| (k, v.len())).collect();
    let highlights_total: usize = highlights_count.values().into_iter().sum();
    let premieres_count: HashMap<_, _> = premieres.iter().map(|(k, v)| (k, v.len())).collect();
    let premieres_total: usize = premieres_count.values().into_iter().sum();
    let uploads_count: HashMap<_, _> = uploads.iter().map(|(k, v)| (k, v.len())).collect();
    let uploads_total: usize = uploads_count.values().into_iter().sum();
    let clips_count: HashMap<_, _> = clips.iter().map(|(k, v)| (k, v.len())).collect();
    let clips_total: usize = clips_count.values().into_iter().sum();

    let counts: HashMap<_, _> = users
        .iter()
        .map(|f| {
            (
                f.to_owned(),
                (
                    vods_count.get(f).unwrap_or(&0),
                    &0usize, // chat_count.get(f).unwrap_or(&0),
                    highlights_count.get(f).unwrap_or(&0),
                    premieres_count.get(f).unwrap_or(&0),
                    uploads_count.get(f).unwrap_or(&0),
                    clips_count.get(f).unwrap_or(&0),
                ),
            )
        })
        .collect();

    // we go by the order in the config, not whatever arbitrary order the hashmap may give
    for k in &users {
        let v = counts.get(k).unwrap();
        println!(
            "{}: {} Vods, {} Chatlogs, {} Highlights, {} Premieres, {} Uploads, {} Clips",
            k,
            v.0,
            v.1,
            v.2,
            v.3,
            v.4,
            v.5,
        );
    }
    println!("Total Vods: {}", vods_total);
    println!("Total Chatlogs: {}", 0usize);
    println!("Total Highlights: {}", highlights_total);
    println!("Total Premieres: {}", premieres_total);
    println!("Total Uploads: {}", uploads_total);
    println!("Total Clips: {}", clips_total);
    println!("");

    // create a new client (for making generic http requests)
    let genclient = reqwest::blocking::Client::new();

    // now to download each set of videos per user
    for k in &users {
        println!("Pulling videos for `{}` ...", k);
        let dir = &conf.directories;

        download_stuff::<Vod>(
            dir.vods.clone(),
            k,
            &mut vods,
            twitch::get_videos_playback_access_tokens,
            itd::download_vod,
            &conf,
            &client,
            &genclient,
        )?;

        download_stuff::<Clip>(
            dir.clips.clone(),
            k,
            &mut clips,
            twitch::get_clips_playback_access_tokens,
            itd::download_clip,
            &conf,
            &client,
            &genclient,
        )?;
    }

    // println!("Done!");

    Ok(())
}

fn download_stuff<T: VodBotData + serde::Serialize>(
    output_dir: PathBuf,
    user_id: &String,
    content: &mut HashMap<String, Vec<T>>,
    token_method: impl FnOnce(
        &GQLClient,
        Vec<String>,
    ) -> Result<HashMap<String, PlaybackAccessToken>, ExitMsg>,
    download_method: impl Fn(&Config, T, PlaybackAccessToken, PathBuf, &Client) -> Result<T, ExitMsg>,
    conf: &Config,
    gqlclient: &GQLClient,
    genclient: &Client,
) -> Result<(), ExitMsg> {
    let content = content.remove(user_id).unwrap();
    let tokens = token_method(gqlclient, content.iter().map(|f| f.identifier()).collect())?;

    for c in content {
        let mut output_path = output_dir.clone().join(c.filename());
        let token = tokens.get(&c.identifier()).unwrap().to_owned();
        let c = download_method(conf, c, token, output_path.clone(), genclient)?;

        output_path.set_extension("meta.json");
        let file = std::fs::File::create(output_path).map_err(|why| ExitMsg {
            code: ExitCode::PullCannotOpenMeta,
            msg: format!("Failed to open meta to write, reason `{}`.", why),
        })?;
        serde_json::to_writer(file, &c).unwrap();
    }
    println!("");

    Ok(())
}
