// Twitch library for making specific queries given a GQLClient

use std::collections::HashMap;

use crate::gql::GQLClient;
use crate::twitch_api::{
    TwitchClipResponse, TwitchPlaybackAccessTokenResponse, TwitchPlaybackAccessTokenToken,
    TwitchUser, TwitchUserResponse, TwitchVideo, TwitchVideoResponse,
};
use crate::util::ExitMsg;
use crate::vodbot_api::{ChatMessage, Clip, PlaybackAccessToken, Vod, VodChapter};

use indoc::formatdoc;

#[derive(Debug, Clone)]
struct QueryMap {
    next: bool,
    id: String,
    after: String,
}

macro_rules! mac_request {
    ($query:expr, $client:ident, $var:ident, $ret:ident, $jq:ident, $tf:expr) => {{
        let mut queries: HashMap<String, QueryMap> = $var
            .iter()
            .map(|f| {
                (
                    "_".to_owned() + &f.replace("-", "_"),
                    QueryMap {
                        next: true,
                        id: f.clone(),
                        after: "".to_owned(),
                    },
                )
            })
            .collect();
        let mut results: HashMap<String, Vec<$ret>> = $var
            .iter()
            .map(|f| ("_".to_owned() + &f.replace("-", "_"), Vec::new()))
            .collect();

        loop {
            let q: Vec<_> = queries
                .values()
                .cloned()
                .filter(|f| f.next)
                .map(|f| formatdoc!($query, f.id.replace("-", "_"), f.id, f.after))
                .collect();

            let j: $jq = $client.query(format!("{{ {} }}", q.join("\n")))?;

            for (k, v) in j.data.unwrap() {
                let q = queries.get_mut(&k).unwrap();
                let r = results.get_mut(&k).unwrap();

                (q.next, q.after) = $tf(&v, r)?;
            }

            if !queries.values().any(|f| f.next) {
                break;
            }
        }

        let results: HashMap<String, Vec<$ret>> = results
            .iter()
            .map(|(k, v)| (k[1..].to_owned().replace("_", "-"), v.to_owned()))
            .collect();

        Ok(results)
    }};
}

pub fn get_channels_videos(
    client: &GQLClient,
    user_logins: Vec<String>,
) -> Result<HashMap<String, Vec<Vod>>, ExitMsg> {
    // Get all videos from a list of channels

    mac_request!(
        "
        _{}: user( login: \"{}\" ) {{
            id login displayName
            videos( after: \"{}\", first: 100, sort: TIME ) {{
                pageInfo {{ hasNextPage }}
                edges {{ cursor node {{
                    id title createdAt status
                    broadcastType lengthSeconds
                    game {{ id name }}
        }}  }}  }}  }}",
        client,
        user_logins,
        Vod,
        TwitchUserResponse,
        |v: &TwitchUser, r: &mut Vec<Vod>| {
            let u = v.videos.as_ref().unwrap();
            let mut after = "".to_owned();

            // For each Vod, lets get it's vod chapters now too
            let vod_ids: Vec<_> = u.edges.iter().map(|f| f.node.id.clone()).collect();
            let chapters = get_videos_chapters(client, vod_ids)?;

            for s in &u.edges {
                let c = chapters.get(&s.node.id).unwrap().to_owned();
                r.push(Vod::from_data(
                    v.id.clone(),
                    v.login.clone(),
                    v.display_name.clone(),
                    &s.node,
                    c,
                ));

                if let Some(c) = s.cursor.to_owned() {
                    after = c;
                }
            }

            Ok((u.page_info.has_next_page, after))
        }
    )
}

pub fn get_channel_videos(client: &GQLClient, user_login: String) -> Result<Vec<Vod>, ExitMsg> {
    Ok(get_channels_videos(client, vec![user_login])?
        .values()
        .last()
        .unwrap()
        .to_owned())
}

pub fn get_channels_clips(
    client: &GQLClient,
    user_logins: Vec<String>,
) -> Result<HashMap<String, Vec<Clip>>, ExitMsg> {
    // Get all clips from a list of channels

    mac_request!(
        "
        _{}: user( login: \"{}\" ) {{
            id login displayName
            clips(
                after: \"{}\", first: 100,
                criteria: {{ period: ALL_TIME, sort: VIEWS_DESC }}
            ) {{
                pageInfo {{ hasNextPage }}
                edges {{ cursor node {{
                    id slug title createdAt viewCount
                    durationSeconds videoOffsetSeconds
                    video {{ id }}
                    game {{ id name }}
                    curator {{ id displayName login }}
        }}  }}  }}  }}",
        client,
        user_logins,
        Clip,
        TwitchUserResponse,
        |v: &TwitchUser, r: &mut Vec<Clip>| {
            let u = v.clips.as_ref().unwrap();
            let mut after = "".to_owned();

            for s in &u.edges {
                r.push(Clip::from_data(
                    v.id.clone(),
                    v.login.clone(),
                    v.display_name.clone(),
                    &s.node,
                ));

                if let Some(c) = s.cursor.to_owned() {
                    after = c;
                }
            }

            Ok((u.page_info.has_next_page, after))
        }
    )
}

pub fn get_channel_clips(client: &GQLClient, user_login: String) -> Result<Vec<Clip>, ExitMsg> {
    Ok(get_channels_clips(client, vec![user_login])?
        .values()
        .last()
        .unwrap()
        .to_owned())
}

pub fn get_videos_comments(
    client: &GQLClient,
    video_ids: Vec<String>,
) -> Result<HashMap<String, Vec<ChatMessage>>, ExitMsg> {
    // Get all videos from a list of channels

    mac_request!(
        "
        _{}: video( id: \"{}\" ) {{
            id title createdAt broadcastType status lengthSeconds
            comments( after: \"{}\", contentOffsetSeconds: 0 ) {{
                pageInfo {{ hasNextPage }}
                edges {{ cursor node {{
                    contentOffsetSeconds
                    commenter {{ displayName }}
                    message {{ fragments {{ mention {{ displayName }} text }} userColor }}
        }}  }}  }}  }}",
        client,
        video_ids,
        ChatMessage,
        TwitchVideoResponse,
        |v: &TwitchVideo, r: &mut Vec<ChatMessage>| {
            let u = v.comments.as_ref().unwrap();
            let mut after = "".to_owned();

            for s in &u.edges {
                r.push(ChatMessage::from_data(&s.node));

                if let Some(c) = s.cursor.to_owned() {
                    after = c;
                }
            }

            Ok((u.page_info.has_next_page, after))
        }
    )
}

pub fn get_video_comments(
    client: &GQLClient,
    video_id: String,
) -> Result<Vec<ChatMessage>, ExitMsg> {
    Ok(get_videos_comments(client, vec![video_id])?
        .values()
        .last()
        .unwrap()
        .to_owned())
}

pub fn get_videos_chapters(
    client: &GQLClient,
    video_ids: Vec<String>,
) -> Result<HashMap<String, Vec<VodChapter>>, ExitMsg> {
    // Get all videos from a list of channels

    mac_request!(
        "
        _{}: video( id: \"{}\" ) {{
            id title createdAt broadcastType status lengthSeconds
            moments(
                after: \"{}\", first: 100,
                momentRequestType: VIDEO_CHAPTER_MARKERS
            ) {{
                pageInfo {{ hasNextPage }}
                edges {{ cursor node {{
                    description
                    positionMilliseconds
                    durationMilliseconds
        }}  }}  }}  }}",
        client,
        video_ids,
        VodChapter,
        TwitchVideoResponse,
        |v: &TwitchVideo, r: &mut Vec<VodChapter>| {
            let u = v.moments.as_ref().unwrap();
            let mut after = "".to_owned();

            for s in &u.edges {
                let n = &s.node;

                r.push(VodChapter {
                    description: n.description.to_owned(),
                    position: n.position_milliseconds / 1000,
                    duration: n.duration_milliseconds / 1000,
                });

                if let Some(c) = s.cursor.to_owned() {
                    after = c;
                }
            }

            Ok((u.page_info.has_next_page, after))
        }
    )
}

pub fn get_video_chapters(
    client: &GQLClient,
    video_id: String,
) -> Result<Vec<VodChapter>, ExitMsg> {
    Ok(get_videos_chapters(client, vec![video_id])?
        .values()
        .last()
        .unwrap()
        .to_owned())
}

pub fn get_videos_playback_access_tokens(
    client: &GQLClient,
    video_ids: Vec<String>,
) -> Result<HashMap<String, PlaybackAccessToken>, ExitMsg> {
    let j = mac_request!(
        "
        _{}: video(id: \"{}{}\") {{
            playbackAccessToken(
                params: {{platform:\"web\",playerType:\"site\",playerBackend:\"mediaplayer\"}}
            ) {{ value signature }}
        }}",
        client,
        video_ids,
        PlaybackAccessToken,
        TwitchPlaybackAccessTokenResponse,
        |v: &TwitchPlaybackAccessTokenToken, r: &mut Vec<PlaybackAccessToken>| {
            let u = &v.playback_access_token;

            r.push(PlaybackAccessToken {
                value: u.value.to_owned(),
                signature: u.signature.to_owned(),
            });

            Ok((false, "".to_owned()))
        }
    )?;

    Ok(j.into_iter()
        .map(|(k, v)| (k, v.last().unwrap().to_owned()))
        .collect())
}

pub fn get_video_playback_access_token(
    client: &GQLClient,
    video_id: String,
) -> Result<PlaybackAccessToken, ExitMsg> {
    Ok(get_videos_playback_access_tokens(client, vec![video_id])?
        .values()
        .last()
        .unwrap()
        .to_owned())
}

pub fn get_clips_playback_access_tokens(
    client: &GQLClient,
    clip_slugs: Vec<String>,
) -> Result<HashMap<String, PlaybackAccessToken>, ExitMsg> {
    let j = mac_request!(
        "
        _{}: clip(slug: \"{}{}\") {{
            playbackAccessToken(
                params: {{platform:\"web\",playerType:\"site\",playerBackend:\"mediaplayer\"}}
            ) {{ value signature }}
        }}",
        client,
        clip_slugs,
        PlaybackAccessToken,
        TwitchPlaybackAccessTokenResponse,
        |v: &TwitchPlaybackAccessTokenToken, r: &mut Vec<PlaybackAccessToken>| {
            let u = &v.playback_access_token;

            r.push(PlaybackAccessToken {
                value: u.value.to_owned(),
                signature: u.signature.to_owned(),
            });

            Ok((false, "".to_owned()))
        }
    )?;

    Ok(j.into_iter()
        .map(|(k, v)| (k, v.last().unwrap().to_owned()))
        .collect())
}

pub fn get_clip_playback_access_token(
    client: &GQLClient,
    clip_slug: String,
) -> Result<PlaybackAccessToken, ExitMsg> {
    Ok(get_clips_playback_access_tokens(client, vec![clip_slug])?
        .values()
        .last()
        .unwrap()
        .to_owned())
}

pub fn _get_channel(client: &GQLClient, user_login: String) -> Result<(), ExitMsg> {
    // Single query
    // Get channel info

    let q = formatdoc! {"
        {{  _: user( login: \"{}\" ) {{
            id login displayName
            description createdAt
            roles {{ isAffiliate isPartner }}
            stream {{
                id title type
                viewersCount
                createdAt 
                game {{ id name }}
        }}  }}  }}", user_login
    };

    let _j: TwitchUserResponse = client.query(q)?;

    Ok(())
}

pub fn _get_video(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
    // Single query
    // Get video info

    let q = formatdoc! {"
        {{  _: video( id: \"{}\" ) {{
            id title publishedAt
            broadcastType lengthSeconds
            game {{ id name }}
            creator {{ id login displayName }}
        }}  }}", video_id
    };

    let _j: TwitchVideoResponse = client.query(q)?;

    Ok(())
}

pub fn _get_clip(client: &GQLClient, clip_slug: String) -> Result<(), ExitMsg> {
    // Single query
    // Get clip info

    let q = formatdoc! {"
        {{  _: clip( slug: \"{}\" ) {{
            id slug title createdAt viewCount
            durationSeconds videoOffsetSeconds
            video {{ id }}
            game {{ id name }}
            videoQualities {{ frameRate quality sourceURL }}
            broadcaster {{ id displayName login }}
            curator {{ id displayName login }}
        }}  }}", clip_slug
    };

    let _j: TwitchClipResponse = client.query(q)?;

    Ok(())
}
