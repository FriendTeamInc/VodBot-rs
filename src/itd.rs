// Independent Twitch Downloader, aka the module where it happens
// a bunch of functions that make it easy to to download videos from Twitch

use std::io::{stdout, Write};
use std::path::PathBuf;
use std::time::Duration;

use m3u8_rs::Playlist;
use reqwest::blocking::Client;

use crate::config::Config;
use crate::util::{chdir, create_dir, format_size, ExitCode, ExitMsg};
use crate::vodbot_api::{Clip, PlaybackAccessToken, Vod};

pub fn download_vod(
    conf: &Config,
    vod: Vod,
    token: PlaybackAccessToken,
    output_path: PathBuf,
    client: &Client,
) -> Result<Vod, ExitMsg> {
    print!("\rVod `{}` ...", vod.id);
    stdout().flush().unwrap();

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

    let playlist_path = temp_dir.clone().join("playlist.m3u8");
    let segment_uri_paths: Vec<_> = p
        .segments
        .iter()
        .map(|f| (temp_dir.clone().join(f.uri.clone()), uri.clone() + &f.uri))
        .collect();

    // then we start the workers on downloading each segment
    std::fs::write(&playlist_path, &bytes).map_err(|why| ExitMsg {
        code: ExitCode::PullCannotWriteSourcePlaylist,
        msg: format!(
            "Failed to use write M3U8 playlist to disk, reason \"{}\".",
            why
        ),
    })?;
    workers_download(conf, &vod, segment_uri_paths, client)?;

    // once the download is done, we spawn an ffmpeg process to stitch it all together
    let currdir = std::env::current_dir().unwrap(); // TODO: this is dangerous, we should fix this.
    chdir(temp_dir)?;
    let loglevel = format!("{:?}", conf.export.ffmpeg_loglevel).to_lowercase();
    let status = std::process::Command::new("ffmpeg")
        .args([
            "-i",
            playlist_path.to_str().unwrap(),
            "-max_interleave_delta",
            "0",
            "-c",
            "copy",
            output_path.to_str().unwrap(),
            "-y",
            "-stats",
            "-loglevel",
            &loglevel,
        ])
        .status()
        .map_err(|why| ExitMsg {
            code: ExitCode::CannotStartFfmpeg,
            msg: format!("Failed to start FFMPEG, reason \"{}\".", why),
        })?;
    chdir(&currdir)?;
    // TODO: sometimes segments are called corrupt by ffmpeg
    // most of the time theyre useable, depending on the version of ffmpeg
    // the streams seem otherwise fine, but maybe we should figure out whats going wrong?

    // check that ffmpeg returned as expected, raise error if necessary
    let status = status.code();
    if let Some(s) = status {
        if s != 0 {
            return Err(ExitMsg {
                code: ExitCode::FfmpegReturnedError,
                msg: format!("FFMPEG returned a non-zero status, `{}`.", s),
            });
        }
    } else if let None = status {
        return Err(ExitMsg {
            code: ExitCode::FfmpegInterrupted,
            msg: format!("FFMPEG was interrupted, no other error."),
        });
    }

    // clear out the temp folder, and we're done here!
    std::fs::remove_dir_all(temp_dir).map_err(|why| ExitMsg {
        code: ExitCode::PullCannotCleanUpAfterDownload,
        msg: format!("Failed to clean up after Vod download, reason \"{}\".", why),
    })?;

    Ok(vod)
}

pub fn download_clip(
    conf: &Config,
    clip: Clip,
    token: PlaybackAccessToken,
    output_path: PathBuf,
    client: &Client,
) -> Result<(), ExitMsg> {
    print!("\rClip `{}` ...", clip.slug);
    stdout().flush().unwrap();

    // get the cdn url
    let url = reqwest::Url::parse_with_params(
        &clip.source_url,
        &[("token", token.value), ("sig", token.signature)],
    )
    .unwrap();

    // download using this url
    let start_time = std::time::Instant::now();
    let size = download_file(
        url.to_string(),
        output_path,
        conf.pull.connection_timeout,
        client,
    )?;

    // print stats
    println!(
        "\rClip `{}` -- {: >8} in {:.1} seconds",
        clip.slug,
        format_size(size, 1, true),
        start_time.elapsed().as_secs_f32()
    );

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
        // TODO: Change this duration?
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
    vod: &Vod,
    paths: Vec<(PathBuf, String)>,
    client: &Client,
) -> Result<(), ExitMsg> {
    let executor = threadpool::ThreadPool::new(conf.pull.download_workers);

    let timeout = conf.pull.connection_timeout;

    let (tx, rx) = std::sync::mpsc::channel();

    let c = std::sync::Arc::new(client.to_owned());

    let total_count = paths.len();

    let start_time = std::time::Instant::now();

    for (p, u) in paths {
        let tx = tx.clone();
        let c = c.clone();
        executor.execute(move || {
            tx.send(download_file(u, p, timeout.clone(), &c))
                .expect("YOU SHOULDN'T BE ABLE TO SEE THIS!");
        });
    }

    let mut done_count: usize = 0;
    let mut dl_size: usize = 0;
    loop {
        done_count += 1;
        let r = rx.recv().unwrap();

        // TODO: proper error checking
        dl_size += r.as_ref().unwrap();
        let perc = (done_count as f32) / (total_count as f32);
        let est_size = ((dl_size as f32) / perc) as usize;
        let duration = start_time.elapsed();
        let d32 = duration.as_secs_f32();
        let dl_speed = ((dl_size as f32) / d32) as usize;
        let time_left = ((total_count - done_count) as f32) * d32 / (done_count as f32);

        if done_count >= total_count {
            print!("\r{}", " ".to_owned().repeat(80));
            println!(
                "\rVod `{}` -- {: >8} in {:.1} seconds",
                vod.id,
                // perc * 100f32,
                format_size(dl_size, 1, true),
                d32
            );
            break;
        } else {
            print!("\r{}", " ".to_owned().repeat(80));
            print!(
                "\rVod `{}` -- {: >3.0}% -- {: >8} of {: >8} (@ {: >8}/s) -- ({: >4.0}s left)",
                vod.id,
                perc * 100f32,
                format_size(dl_size, 1, true),
                format_size(est_size, 1, true),
                format_size(dl_speed, 1, true),
                time_left
            );
            stdout().flush().unwrap();
        }
    }

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
