// Pull command, for grabbing videos off of Twitch

use crate::cli::PullMode;
use crate::config::{load_config, Config, ConfigChannel};
use crate::gql::GQLClient;
use crate::itd;
use crate::twitch;
use crate::util::{ExitCode, ExitMsg, create_dir, get_meta_ids};
use crate::vodbot_api::{Clip, PlaybackAccessToken, Vod, VodBotData, ChatMessage, ChatLog};

use reqwest::blocking::Client;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn run(config_path: PathBuf, _mode: PullMode) -> Result<(), ExitMsg> {
    let conf = load_config(&config_path)?;
    let c = &conf.channels;

    let f = |f: &ConfigChannel| f.username.clone();
    let users: Vec<_> = c.iter().map(f).collect();
    println!("Checking users: {} ...", users.join(", "));

    // TODO: do save_* checks?

    let client = GQLClient::new(conf.pull.gql_client_id.clone());

    let mut vods = twitch::get_channels_videos_archive(&client, &users)?;
    let mut highlights = twitch::get_channels_videos_highlight(&client, &users)?;
    let mut premieres = twitch::get_channels_videos_premiere(&client, &users)?;
    let mut uploads = twitch::get_channels_videos_upload(&client, &users)?;
    let mut clips = twitch::get_channels_clips(&client, &users)?;

    let mut chat = HashMap::<String, Vec<ChatLog>>::new();
    for k in &users {
        let vods = vods.get(k).unwrap();
        let vod_ids: Vec<_> = vods.iter().map(|f| f.id).collect();
        let v = twitch::get_videos_comments(&client, &vod_ids)?;
        // TODO: move this bit into twitch.rs?
        let v: Vec<_> = v.into_iter().map(|(u, v)| {
            ChatLog {
                video_id: u,
                messages: v,
            }
        }).collect();
        chat.insert(k.clone(), v);
    }

    for k in &users {
        let dir = &conf.directories;
        let d = (
            get_meta_ids(dir.vods.clone().join(&k))?,
            get_meta_ids(dir.highlights.clone().join(&k))?,
            get_meta_ids(dir.premieres.clone().join(&k))?,
            get_meta_ids(dir.uploads.clone().join(&k))?,
            get_meta_ids(dir.clips.clone().join(&k))?,
            get_meta_ids(dir.chat.clone().join(&k))?,
        );
        d.0.into_iter().for_each(|v| vods.get_mut(k).unwrap().retain(|f| f.id != v));
        d.1.into_iter().for_each(|v| highlights.get_mut(k).unwrap().retain(|f| f.id != v));
        d.2.into_iter().for_each(|v| premieres.get_mut(k).unwrap().retain(|f| f.id != v));
        d.3.into_iter().for_each(|v| uploads.get_mut(k).unwrap().retain(|f| f.id != v));
        d.4.into_iter().for_each(|v| clips.get_mut(k).unwrap().retain(|f| f.slug != v));
        // d.5.into_iter().for_each(|v| chat.get_mut(k).unwrap().retain(|f| f.id != v));
    }

    let vods_count: HashMap<_, _> = vods.iter().map(|(k, v)| (k, v.len())).collect();
    let vods_total: usize = vods_count.values().into_iter().sum();
    let highlights_count: HashMap<_, _> = highlights.iter().map(|(k, v)| (k, v.len())).collect();
    let highlights_total: usize = highlights_count.values().into_iter().sum();
    let premieres_count: HashMap<_, _> = premieres.iter().map(|(k, v)| (k, v.len())).collect();
    let premieres_total: usize = premieres_count.values().into_iter().sum();
    let uploads_count: HashMap<_, _> = uploads.iter().map(|(k, v)| (k, v.len())).collect();
    let uploads_total: usize = uploads_count.values().into_iter().sum();
    let clips_count: HashMap<_, _> = clips.iter().map(|(k, v)| (k, v.len())).collect();
    let clips_total: usize = clips_count.values().into_iter().sum();
    let chat_count: HashMap<_, _> = chat.iter().map(|(k, v)| (k, v.len())).collect();
    let chat_total: usize = chat_count.values().into_iter().sum();
    let total_total: usize = vods_total + highlights_total + premieres_total + uploads_total + clips_total + chat_total;

    let counts: HashMap<_, _> = users
        .iter()
        .map(|f| {
            (
                f.to_owned(),
                (
                    vods_count.get(f).unwrap_or(&0).clone(),
                    highlights_count.get(f).unwrap_or(&0).clone(),
                    premieres_count.get(f).unwrap_or(&0).clone(),
                    uploads_count.get(f).unwrap_or(&0).clone(),
                    clips_count.get(f).unwrap_or(&0).clone(),
                    chat_count.get(f).unwrap_or(&0).clone(),
                ),
            )
        })
        .collect();

    // we go by the order in the config, not whatever arbitrary order the hashmap may give
    for k in &users {
        let v = counts.get(k).unwrap();
        println!(
            "{}: {} Vods, {} Highlights, {} Premieres, {} Uploads, {} Clips, {} Chatlogs",
            k, v.0, v.1, v.2, v.3, v.4, v.5,
        );
    }
    println!("Total Vods: {}", vods_total);
    println!("Total Highlights: {}", highlights_total);
    println!("Total Premieres: {}", premieres_total);
    println!("Total Uploads: {}", uploads_total);
    println!("Total Clips: {}", clips_total);
    println!("Total Chatlogs: {}", chat_total);
    println!("Total: {}", total_total);
    println!("");

    // create a new client (for making generic http requests)
    let genclient = reqwest::blocking::Client::new();

    // now to download each set of videos per user
    let dir = &conf.directories;
    for k in &users {
        let count = counts.get(k).unwrap_or(&(0, 0, 0, 0, 0, 0));
        let user_total = count.0 + count.1 + count.2 + count.3 + count.4 + count.5;
        if user_total == 0 {
            break;
        }

        println!("Pulling {} videos for `{}` ...", user_total, k);

        // Vods
        download_stuff::<Vod>(
            dir.vods.clone(), k, &mut vods,
            twitch::get_videos_playback_access_tokens,
            itd::download_vod, &conf, &client, &genclient,
            "Vod".to_owned(),
        )?;
        // Chatlogs
        // TODO

        // Highlights
        download_stuff::<Vod>(
            dir.highlights.clone(), k, &mut highlights,
            twitch::get_videos_playback_access_tokens,
            itd::download_vod, &conf, &client, &genclient,
            "Highlight".to_owned(),
        )?;
        // Premiere
        download_stuff::<Vod>(
            dir.premieres.clone(), k, &mut premieres,
            twitch::get_videos_playback_access_tokens,
            itd::download_vod, &conf, &client, &genclient,
            "Premiere".to_owned(),
        )?;
        // Upload
        download_stuff::<Vod>(
            dir.uploads.clone(), k, &mut uploads,
            twitch::get_videos_playback_access_tokens,
            itd::download_vod, &conf, &client, &genclient,
            "Upload".to_owned(),
        )?;
        // Clip
        download_stuff::<Clip>(
            dir.clips.clone(), k, &mut clips,
            twitch::get_clips_playback_access_tokens,
            itd::download_clip, &conf, &client, &genclient,
            "Clip".to_owned(),
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
        &Vec<String>,
    ) -> Result<HashMap<String, PlaybackAccessToken>, ExitMsg>,
    download_method: impl Fn(&Config, T, PlaybackAccessToken, PathBuf, &Client, String) -> Result<T, ExitMsg>,
    conf: &Config,
    gqlclient: &GQLClient,
    genclient: &Client,
    noun: String,
) -> Result<(), ExitMsg> {
    let content = content.remove(user_id).unwrap();
    let tokens = token_method(gqlclient, &content.iter().map(|f| f.identifier()).collect())?;

    let mut has_content = false;
    for c in content {
        has_content = true;
        let output_path = output_dir.clone().join(user_id);
        create_dir(&output_path)?;
        let mut output_path = output_path.join(c.filename());
        let token = tokens.get(&c.identifier()).unwrap().to_owned();
        let c = download_method(conf, c, token, output_path.clone(), genclient, noun.clone())?;

        output_path.set_extension("meta.json");
        let file = std::fs::File::create(output_path).map_err(|why| ExitMsg {
            code: ExitCode::PullCannotOpenMeta,
            msg: format!("Failed to open meta to write, reason `{}`.", why),
        })?;
        serde_json::to_writer(file, &c).unwrap();
    }
    if has_content {
        println!("");
    }

    Ok(())
}
