// Independent Twitch Downloader, aka the module where it happens
// a bunch of functions that make it easy to to download videos from Twitch

use crate::config::Config;
use crate::vodbot_api::{Clip, Vod};
use crate::util::ExitMsg;

pub fn download_vod(conf: &Config, vod: Vod) -> Result<(), ExitMsg> {
    println!("Downloading VOD {}", vod.id);
    Ok(())
}

pub fn download_clip(conf: &Config, clip: Clip) -> Result<(), ExitMsg> {
    println!("Downloading Clip {}", clip.slug);
    Ok(())
}

