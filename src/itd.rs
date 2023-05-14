// Independent Twitch Downloader, aka the module where it happens
// a bunch of functions that make it easy to to download videos from Twitch

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use futures::executor::ThreadPool;
use m3u8_rs::Playlist;
use reqwest::blocking::Client;

use crate::cli::Cli;
use crate::config::Config;
use crate::util::{create_dir, ExitCode, ExitMsg};
use crate::vodbot_api::{Clip, PlaybackAccessToken, Vod};

pub fn download_vods(
    conf: &Config,
    vods: Vec<Vod>,
    tokens: HashMap<String, PlaybackAccessToken>,
    client: &Client,
) -> Result<(), ExitMsg> {
    for v in vods {
        // TODO: create exitmsg if missing token, or just generic message print?
        let token = tokens.get(&v.id).unwrap().to_owned();
        download_vod(conf, v, token, client)?;
    }

    Ok(())
}

pub fn download_vod(
    conf: &Config,
    vod: Vod,
    token: PlaybackAccessToken,
    client: &Client,
) -> Result<(), ExitMsg> {
    println!("Downloading VOD {}", vod.id);

    // get m3u8 quality playlist, first uri is the source quality
    let mut uri = get_playlist_source_uri(&vod, token, client)?;

    // then we use that uri to grab the video segment playlist, also m3u8
    let resp = client.get(&uri).send().map_err(|why| ExitMsg {
        code: ExitCode::PullCannotGetSourcePlaylist,
        msg: format!("Failed to get source M3U8 playlist, reason: \"{}\".", why,),
    })?;
    let bytes = resp.bytes().map_err(|why| ExitMsg {
        code: ExitCode::PullCannotReadSourcePlaylist,
        msg: format!("Failed to read source M3U8 playlist, reason: \"{}\".", why,),
    })?;
    let playlist = m3u8_rs::parse_playlist(&bytes.clone())
        .map_err(|why| ExitMsg {
            code: ExitCode::PullCannotParseSourcePlaylist,
            msg: format!("Failed to parse source M3U8 playlist, reason: \"{}\".", why,),
        })?
        .1;

    let p = match playlist {
        Playlist::MediaPlaylist(p) => Ok(p),
        _ => Err(ExitMsg {
            code: ExitCode::PullCannotUseSourcePlaylist,
            msg: format!("Failed to use source M3U8 playlist."),
        }),
    }?;

    // then we determine what paths each segment should have
    let temp_dir = &conf.directories.temp.clone().join(vod.id.clone());
    create_dir(temp_dir)?;

    let split_idx = uri.clone().rfind("/").unwrap() + 1;
    uri.split_off(split_idx).truncate(split_idx);
    println!("{}", uri);

    let playlist_path = temp_dir.clone().join("playlist.m3u8");
    let segment_uri_paths: Vec<_> = p
        .segments
        .iter()
        .map(|f| (temp_dir.clone().join(f.uri.clone()), uri.clone() + &f.uri))
        .collect();

    // then we start the workers on downloading each segment
    std::fs::write(playlist_path, &bytes).map_err(|why| ExitMsg {
        code: ExitCode::PullCannotWriteSourcePlaylist,
        msg: format!(
            "Failed to use write M3U8 playlist to disk, reason \"{}\".",
            why
        ),
    })?;

    // once the download is done, we spawn an ffmpeg process to stitch it all back together

    // check that ffmpeg returned as expected, raise error if necessary

    // clear out the temp folder, and we're done here!
    std::fs::remove_dir_all(temp_dir).map_err(|why| ExitMsg {
        code: ExitCode::PullCannotCleanUpAfterDownload,
        msg: format!("Failed to clean up after Vod download, reason \"{}\".", why),
    })?;

    Ok(())
}

pub fn download_clips(
    conf: &Config,
    clips: Vec<Clip>,
    tokens: HashMap<String, PlaybackAccessToken>,
    client: &Client,
) -> Result<(), ExitMsg> {
    for c in clips {
        // TODO: create exitmsg if missing token, or just generic message print?
        let token = tokens.get(&c.slug).unwrap().to_owned();
        download_clip(conf, c, token, client)?;
    }

    Ok(())
}

pub fn download_clip(
    conf: &Config,
    clip: Clip,
    token: PlaybackAccessToken,
    client: &Client,
) -> Result<(), ExitMsg> {
    println!("Downloading Clip {}", clip.slug);

    Ok(())
}

fn get_playlist_source_uri(
    vod: &Vod,
    token: PlaybackAccessToken,
    client: &Client,
) -> Result<String, ExitMsg> {
    let url = reqwest::Url::parse_with_params(
        format!("http://usher.ttvnw.net/vod/{}", vod.id).as_str(),
        &[
            ("nauth", token.value),
            ("nauthsig", token.signature),
            ("allow_source", "true".to_owned()),
            ("player", "twitchweb".to_owned()),
        ],
    )
    .unwrap();
    let resp = client
        .get(url)
        .timeout(Duration::from_secs(5))
        .send()
        .map_err(|why| ExitMsg {
            code: ExitCode::PullCannotGetPlaylistURI,
            msg: format!(
                "Failed to get M3U8 playlist from Twitch, reason: \"{}\".",
                why,
            ),
        })?;

    let bytes = resp.bytes().map_err(|why| ExitMsg {
        code: ExitCode::PullCannotReadPlaylistURI,
        msg: format!(
            "Failed to read M3U8 playlist from Twitch, reason: \"{}\".",
            why,
        ),
    })?;

    let playlist = m3u8_rs::parse_playlist(&bytes)
        .map_err(|why| ExitMsg {
            code: ExitCode::PullCannotParsePlaylistURI,
            msg: format!(
                "Failed to parse M3U8 playlist from Twitch, reason: \"{}\".",
                why,
            ),
        })?
        .1;

    if let Playlist::MasterPlaylist(p) = playlist {
        Ok(p.variants.first().unwrap().uri.to_owned())
    } else {
        Err(ExitMsg {
            code: ExitCode::PullCannotFindPlaylistURI,
            msg: format!("Failed to find source M3U8 playlist URI from Twitch."),
        })
    }
}

fn workers_download(
    conf: &Config,
    paths: Vec<(PathBuf, String)>,
    client: &Client,
) -> Result<(), ExitMsg> {
    let executor = ThreadPool::builder()
        .pool_size(conf.pull.download_workers)
        .create()
        .map_err(|why| ExitMsg {
            code: ExitCode::PullCannotCreateThreadPool,
            msg: format!("Failed to create worker thread pool, reason \"{}\".", why),
        })?;

    let futures = paths.into_iter().map(|(p, u)| executor.spawn_ok(async {}));

    Ok(())
}

fn download_file(
    url: String,
    path: PathBuf,
    timeout: usize,
    client: &Client,
) -> Result<usize, ExitMsg> {
    let resp = client
        .get(url)
        .timeout(Duration::from_secs(timeout as u64))
        .send()
        .map_err(|why| ExitMsg {
            code: ExitCode::PullCannotGetChunk,
            msg: format!("Failed to get file, reason \"{}\".", why),
        })?;

    let bytes = resp.bytes().map_err(|why| ExitMsg {
        code: ExitCode::PullCannotParseChunk,
        msg: format!("Failed to parse file, reason \"{}\".", why),
    })?;

    std::fs::write(path, &bytes).map_err(|why| ExitMsg {
        code: ExitCode::PullCannotWriteChunk,
        msg: format!("Failed to write file, reason \"{}\".", why),
    })?;

    Ok(bytes.len())
}
