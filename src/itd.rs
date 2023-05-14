// Independent Twitch Downloader, aka the module where it happens
// a bunch of functions that make it easy to to download videos from Twitch

use std::collections::HashMap;

use crate::config::Config;
use crate::util::ExitMsg;
use crate::vodbot_api::{Clip, PlaybackAccessToken, Vod};

pub fn download_vods(
    conf: &Config,
    vods: Vec<Vod>,
    tokens: HashMap<String, PlaybackAccessToken>,
) -> Result<(), ExitMsg> {
    for v in vods {
        // TODO: create exitmsg if missing token, or just generic message print?
        let token = tokens.get(&v.id).unwrap().to_owned();
        download_vod(conf, v, token)?;
    }

    Ok(())
}

pub fn download_vod(conf: &Config, vod: Vod, token: PlaybackAccessToken) -> Result<(), ExitMsg> {
    println!("Downloading VOD {}", vod.id);

    // get m3u8 quality playlist, first uri is the source quality

    // then we use that uri to grab the video segment playlist, also m3u8

    // then we determine what paths each segment should have

    // then we start the workers on downloading each segment

    // once the download is done, we spawn an ffmpeg process to stitch it all back together

    // check that ffmpeg returned as expected, raise error if necessary

    // clear out the temp folder, and we're done here!

    Ok(())
}

pub fn download_clips(
    conf: &Config,
    clips: Vec<Clip>,
    tokens: HashMap<String, PlaybackAccessToken>,
) -> Result<(), ExitMsg> {
    for c in clips {
        // TODO: create exitmsg if missing token, or just generic message print?
        let token = tokens.get(&c.slug).unwrap().to_owned();
        download_clip(conf, c, token)?;
    }

    Ok(())
}

pub fn download_clip(conf: &Config, clip: Clip, token: PlaybackAccessToken) -> Result<(), ExitMsg> {
    println!("Downloading Clip {}", clip.slug);

    Ok(())
}

fn get_playlist_uris(vod: Vod, token: PlaybackAccessToken) {}
