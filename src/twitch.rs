// Twitch library for making specific queries given a GQLClient

use std::collections::HashMap;

use crate::gql::GQLClient;
use crate::util::ExitMsg;
use crate::vodbot_api::Vod;

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
                    after_page: String::from(""),
                },
            )
        })
        .collect();
    let mut results: HashMap<String, Vec<Vod>> = user_logins
        .iter()
        .map(|f| (f.clone(), Vec::new()))
        .collect();

    // println!("{}", queries.values().cloned().collect::<Vec<_>>().join("\n"));

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
                                id title publishedAt status
                                broadcastType lengthSeconds
                                game {{ id name }}
                    }}  }}  }}  }}", f.id, f.id, f.after_page
                }
            })
            .collect();

        let j = client.query(format!("{{ {} }}", q.join("\n")))?;

        for (k, v) in j.data.unwrap() {
            let q = queries.get_mut(&k).unwrap();
            let r = results.get_mut(&k).unwrap();

            let u = v.videos.as_ref().unwrap();

            q.has_next_page = u.page_info.has_next_page;

            // For each Vod, lets get it's vod chapters now too
            // TODO: do that, and map them into each new object
            let vod_ids: Vec<_> = u.edges.iter().map(|f| f.node.id.clone()).collect();

            for s in &u.edges {
                r.push(Vod {
                    id: s.node.id.to_owned(),
                    streamer_id: v.id.clone(),
                    streamer_login: v.login.clone(),
                    streamer_name: v.display_name.clone(),
                    game_id: s
                        .node
                        .game
                        .as_ref()
                        .map(|f| Some(f.id.to_owned()))
                        .unwrap_or(None),
                    game_name: s
                        .node
                        .game
                        .as_ref()
                        .map(|f| Some(f.name.to_owned()))
                        .unwrap_or(None),
                    title: s.node.title.to_owned(),
                    created_at: s.node.published_at.to_owned(),
                    chapters: Vec::new(),
                    duration: s.node.length_seconds,
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

pub fn _get_channel_clips(client: &GQLClient, user_login: String) -> Result<(), ExitMsg> {
    // Paged query
    // Get all clips from a channel

    let after = "";
    loop {
        let q = formatdoc! {"
            {{  user( login: \"{}\" ) {{
                id login displayName
                clips(
                    after: \"{}\", first: 100,
                    criteria: {{ period: ALL_TIME, sort: CREATED_AT_DESC }}
                ) {{
                    edges {{ cursor node {{
                        id slug title createdAt viewCount
                        durationSeconds videoOffsetSeconds
                        video {{ id }}
                        game {{ id name }}
                        curator {{ id displayName login }}
            }}  }}  }}  }}  }}", user_login, after
        };

        let _j = client.query(q)?;

        break;
    }

    Ok(())
}

pub fn _get_video_comments(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
    // Paged query
    // Get all "comments" (chat messages) from a video

    let after = "";
    loop {
        let q = formatdoc! {"
            {{  video( id: \"{}\" ) {{
                comments( after: \"{}\", contentOffsetSeconds: 0 ) {{
                    edges {{ cursor node {{
                        contentOffsetSeconds
                        commenter {{ displayName }}
                        message {{ userColor fragments {{ mention {{ displayName }} text }} }}
            }}  }}  }}  }} }}", video_id, after
        };

        let _j = client.query(q)?;

        break;
    }

    Ok(())
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

        let _j = client.query(q)?;

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

    let _j = client.query(q)?;

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

    let _j = client.query(q)?;

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

    let _j = client.query(q)?;

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

    let _j = client.query(q)?;

    Ok(())
}
