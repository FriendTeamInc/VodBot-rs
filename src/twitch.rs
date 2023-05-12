// Twitch library for making specific queries given a GQLClient

use std::collections::HashMap;

use crate::gql::GQLClient;
use crate::twitch_api::{TwitchUserResponse, TwitchVideoResponse};
use crate::util::ExitMsg;
use crate::vodbot_api::{ChatMessage, Clip, Vod};

use indoc::formatdoc;

#[derive(Debug, Clone)]
struct QueryMap {
    has_next_page: bool,
    id: String,
    after_page: String,
}

pub fn get_channels_videos(
    client: &GQLClient,
    user_logins: Vec<String>,
) -> Result<HashMap<String, Vec<Vod>>, ExitMsg> {
    // Paged query
    // Get all videos from a list of channels

    let mut queries: HashMap<String, QueryMap> = user_logins
        .iter()
        .map(|f| {
            (
                f.clone(),
                QueryMap {
                    has_next_page: true,
                    id: f.clone(),
                    after_page: "".to_owned(),
                },
            )
        })
        .collect();
    let mut results: HashMap<String, Vec<Vod>> = user_logins
        .iter()
        .map(|f| (f.clone(), Vec::new()))
        .collect();

    loop {
        let q: Vec<_> = queries
            .values()
            .cloned()
            .filter(|f| f.has_next_page)
            .map(|f| {
                formatdoc! {"
                    {}: user( login: \"{}\" ) {{
                        id login displayName
                        videos( after: \"{}\", first: 100, sort: TIME ) {{
                            pageInfo {{ hasNextPage }}
                            edges {{ cursor node {{
                                id title createdAt status
                                broadcastType lengthSeconds
                                game {{ id name }}
                    }}  }}  }}  }}", f.id, f.id, f.after_page
                }
            })
            .collect();

        let j: TwitchUserResponse = client.query(format!("{{ {} }}", q.join("\n")))?;

        for (k, v) in j.data.unwrap() {
            let q = queries.get_mut(&k).unwrap();
            let r = results.get_mut(&k).unwrap();

            let u = v.videos.as_ref().unwrap();

            q.has_next_page = u.page_info.has_next_page;

            // For each Vod, lets get it's vod chapters now too
            // TODO: do that, and map them into each new object
            let vod_ids: Vec<_> = u.edges.iter().map(|f| f.node.id.clone()).collect();

            for s in &u.edges {
                let n = &s.node;
                let g = &n.game;

                r.push(Vod {
                    id: n.id.to_owned(),
                    streamer_id: v.id.clone(),
                    streamer_login: v.login.clone(),
                    streamer_name: v.display_name.clone(),
                    game_id: g.as_ref().map(|f| f.id.to_owned()).unwrap_or("".to_owned()),
                    game_name: g
                        .as_ref()
                        .map(|f| f.name.to_owned())
                        .unwrap_or("".to_owned()),
                    title: n.title.to_owned(),
                    created_at: n.created_at.to_owned(),
                    chapters: Vec::new(),
                    duration: n.length_seconds,
                    has_chat: false,
                });
                if let Some(c) = s.cursor.to_owned() {
                    q.after_page = c;
                }
            }
        }

        if !queries.values().any(|f| f.has_next_page) {
            break;
        }
    }

    Ok(results)
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
    // Paged query
    // Get all clips from a list of channels

    let mut queries: HashMap<String, QueryMap> = user_logins
        .iter()
        .map(|f| {
            (
                f.clone(),
                QueryMap {
                    has_next_page: true,
                    id: f.clone(),
                    after_page: "".to_owned(),
                },
            )
        })
        .collect();
    let mut results: HashMap<String, Vec<Clip>> = user_logins
        .iter()
        .map(|f| (f.clone(), Vec::new()))
        .collect();

    loop {
        let q: Vec<_> = queries
            .values()
            .cloned()
            .filter(|f| f.has_next_page)
            .map(|f| {
                formatdoc! {"
                    {}: user( login: \"{}\" ) {{
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
                    }}  }}  }}  }}", f.id, f.id, f.after_page
                }
            })
            .collect();

        let j: TwitchUserResponse = client.query(format!("{{ {} }}", q.join("\n")))?;

        for (k, v) in j.data.unwrap() {
            let q = queries.get_mut(&k).unwrap();
            let r = results.get_mut(&k).unwrap();

            let u = v.clips.as_ref().unwrap();

            q.has_next_page = u.page_info.has_next_page;

            for s in &u.edges {
                let n = &s.node;
                let g = &n.game;
                let c = &n.curator;

                r.push(Clip {
                    id: n.id.to_owned(),
                    slug: n.id.to_owned(),
                    streamer_id: v.id.clone(),
                    streamer_login: v.login.clone(),
                    streamer_name: v.display_name.clone(),
                    clipper_id: c.as_ref().map(|f| f.id.to_owned()).unwrap_or("".to_owned()),
                    clipper_login: c
                        .as_ref()
                        .map(|f| f.login.to_owned())
                        .unwrap_or("".to_owned()),
                    clipper_name: c
                        .as_ref()
                        .map(|f| f.display_name.to_owned())
                        .unwrap_or("".to_owned()),
                    game_id: g.as_ref().map(|f| f.id.to_owned()).unwrap_or("".to_owned()),
                    game_name: g
                        .as_ref()
                        .map(|f| f.name.to_owned())
                        .unwrap_or("".to_owned()),
                    title: n.title.to_owned(),
                    created_at: n.created_at.to_owned(),
                    duration: n.duration_seconds,
                    offset: n.video_offset_secconds.unwrap_or(0),
                    view_count: n.view_count,
                    vod_id: n
                        .video
                        .as_ref()
                        .map(|f| f.id.to_owned())
                        .unwrap_or("".to_owned()),
                });
                if let Some(c) = s.cursor.to_owned() {
                    q.after_page = c;
                }
            }
        }

        if !queries.values().any(|f| f.has_next_page) {
            break;
        }
    }

    Ok(results)
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
    // Paged query
    // Get all videos from a list of channels

    let mut queries: HashMap<String, QueryMap> = video_ids
        .iter()
        .map(|f| {
            (
                "_".to_owned() + f,
                QueryMap {
                    has_next_page: true,
                    id: f.clone(),
                    after_page: "".to_owned(),
                },
            )
        })
        .collect();
    let mut results: HashMap<String, Vec<ChatMessage>> = video_ids
        .iter()
        .map(|f| ("_".to_owned() + f, Vec::new()))
        .collect();

    loop {
        let q: Vec<_> = queries
            .values()
            .cloned()
            .filter(|f| f.has_next_page)
            .map(|f| {
                formatdoc! {"
                    _{}: video( id: \"{}\" ) {{
                        id title createdAt broadcastType status lengthSeconds
                        comments( after: \"{}\", contentOffsetSeconds: 0 ) {{
                            pageInfo {{ hasNextPage }}
                            edges {{ cursor node {{
                                contentOffsetSeconds
                                commenter {{ displayName }}
                                message {{ fragments {{ mention {{ displayName }} text }} userColor }}
                    }}  }}  }}  }}", f.id, f.id, f.after_page
                }
            })
            .collect();
        // We grab title, id, etc because it makes managing results easier.
        // It is technically wasted bandwidth. Too bad!
        // TODO: Fix that?

        let j: TwitchVideoResponse = client.query(format!("{{ {} }}", q.join("\n")))?;

        for (k, v) in j.data.unwrap() {
            let q = queries.get_mut(&k).unwrap();
            let r = results.get_mut(&k).unwrap();

            let u = v.comments.as_ref().unwrap();

            q.has_next_page = u.page_info.has_next_page;

            for s in &u.edges {
                let n = &s.node;
                let f = &n.message.fragments;

                r.push(ChatMessage {
                    user_name: n.commenter.display_name.to_owned(),
                    color: n.message.user_color.to_owned().unwrap_or("".to_owned()),
                    offset: n.content_offset_seconds,
                    msg: f.iter().map(|f|
                        f
                        .mention
                        .as_ref()
                        .map(|f| format!("@{} ", f.display_name))
                        .unwrap_or("".to_owned())
                        .to_owned()
                        + &f.text
                    ).collect(), // ::<Vec<String>>().join(" "),
                });

                if let Some(c) = s.cursor.to_owned() {
                    q.after_page = c;
                }
            }
        }

        if !queries.values().any(|f| f.has_next_page) {
            break;
        }
    }

    let results: HashMap<String, Vec<ChatMessage>> = results
        .iter()
        .map(|(k, v)| (k[1..].to_owned(), v.to_owned()))
        .collect();

    Ok(results)
}

pub fn get_video_comments(client: &GQLClient, video_id: String) -> Result<Vec<ChatMessage>, ExitMsg> {
    Ok(get_videos_comments(client, vec![video_id])?
        .values()
        .last()
        .unwrap()
        .to_owned())
}

pub fn _get_video_chapters(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
    // Paged query
    // Get all "chapters" (like game changes) from a video

    let after = "";
    loop {
        let q = formatdoc! {"
            {{  video( id: \"{}\" ) {{
                moments(
                    after: \"{}\", first: 100,
                    momentRequestType: VIDEO_CHAPTER_MARKERS
                ) {{
                    edges {{ cursor node {{
                        description type
                        positionMilliseconds
                        durationMilliseconds
            }}  }}  }}  }}  }}", video_id, after
        };

        // let _j = client.query(q)?;

        break;
    }

    Ok(())
}

pub fn _get_channel(client: &GQLClient, user_login: String) -> Result<(), ExitMsg> {
    // Single query
    // Get channel info

    let q = formatdoc! {"
        {{  user( login: \"{}\" ) {{
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

    // let _j = client.query(q)?;

    Ok(())
}

pub fn _get_video(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
    // Single query
    // Get video info

    let q = formatdoc! {"
        {{  video( id: \"{}\" ) {{
            id title publishedAt
            broadcastType lengthSeconds
            game {{ id name }}
            creator {{ id login displayName }}
        }}  }}", video_id
    };

    // let _j = client.query(q)?;

    Ok(())
}

pub fn _get_clip(client: &GQLClient, clip_slug: String) -> Result<(), ExitMsg> {
    // Single query
    // Get clip info

    let q = formatdoc! {"
        {{  clip( slug: \"{}\" ) {{
            id slug title createdAt viewCount
            durationSeconds videoOffsetSeconds
            video {{ id }}
            game {{ id name }}
            videoQualities {{ frameRate quality sourceURL }}
            broadcaster {{ id displayName login }}
            curator {{ id displayName login }}
        }}  }}", clip_slug
    };

    // let _j = client.query(q)?;

    Ok(())
}

pub fn _get_playback_access_token(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
    // Single query
    // Get playback access token (for downloading videos)

    let q = formatdoc! {"
        {{  videoPlaybackAccessToken(
                id: \"{}\",
                params: {{
                    platform:\"web\", playerType:\"site\",
                    playerBackend:\"mediaplayer\"
                }}
            ) {{ signature value }}
        }}", video_id
    };

    // let _j = client.query(q)?;

    Ok(())
}
