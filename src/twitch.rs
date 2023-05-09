// Twitch library for making specific queries given a GQLClient

use crate::gql::GQLClient;
use crate::twitch_api::TwitchResponseData;
use crate::util::ExitMsg;

use indoc::formatdoc;

pub fn get_channel_videos(client: &GQLClient, user_login: String) -> Result<(), ExitMsg> {
    // Paged query
    // Get all videos from a channel

    let mut after: String = String::from("");
    loop {
        let q = formatdoc! {"
            {{  user( login: \"{}\" ) {{
                videos( after: \"{}\", first: 100, sort: TIME ) {{
                    totalCount pageInfo {{ hasNextPage }}
                    edges {{ cursor node {{
                        id title publishedAt status
                        broadcastType lengthSeconds
                        game {{ id name }}
                        creator {{ id login displayName }}
            }}  }}  }}  }}  }}", user_login, after
        };

        let j = client.query(q)?;

        if let Some(TwitchResponseData::User(u)) = j.data {
            if let Some(t) = u.videos {
                // Round up the videos
                // TODO: convert TwitchUserVideoNode into a more usable format
                let r: Vec<_> = t.edges.iter().map(|f| &f.node).collect();
                println!("{:?}", r);

                // Handle paging
                if t.page_info.has_next_page {
                    after = t.edges.last().unwrap().cursor.clone().unwrap();
                } else {
                    break;
                }
            }
        }
    }

    Ok(())
}

pub fn get_channel_clips(client: &GQLClient, user_login: String) -> Result<(), ExitMsg> {
    // Paged query
    // Get all clips from a channel

    let after = "";
    loop {
        let q = formatdoc! {"
            {{  user( login: \"{}\" ) {{
                clips(
                    after: \"{}\", first: 100,
                    criteria: {{ period: ALL_TIME, sort: CREATED_AT_DESC }}
                ) {{
                    edges {{ cursor node {{
                        id slug title createdAt viewCount
                        durationSeconds videoOffsetSeconds
                        video {{ id }}
                        game {{ id name }}
                        broadcaster {{ id displayName login }}
                        curator {{ id displayName login }}
            }}  }}  }}  }}  }}", user_login, after
        };

        let j = client.query(q)?;

        break;
    }

    Ok(())
}

pub fn get_video_comments(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
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

        let j = client.query(q)?;

        break;
    }

    Ok(())
}

pub fn get_video_chapters(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
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

        let j = client.query(q)?;

        break;
    }

    Ok(())
}

pub fn get_channel(client: &GQLClient, user_login: String) -> Result<(), ExitMsg> {
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

    let j = client.query(q)?;

    Ok(())
}

pub fn get_video(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
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

    let j = client.query(q)?;

    Ok(())
}

pub fn get_clip(client: &GQLClient, clip_slug: String) -> Result<(), ExitMsg> {
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

    let j = client.query(q)?;

    Ok(())
}

pub fn get_playback_access_token(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
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

    let j = client.query(q)?;

    Ok(())
}
