// Twitch library for making specific queries given a GQLClient

use crate::gql::GQLClient;
use crate::twitch_api::TwitchResponseData;
use crate::util::ExitMsg;

pub fn get_channel_videos(client: &GQLClient, user_login: String) -> Result<(), ExitMsg> {
    // Paged query
    // Get all videos from a channel

    let after = "";
    loop {
        let q = format!(
            "{{  user(login: \"{}\") {{
                videos( first: 100, sort: TIME, after: \"{}\" ) {{
                    totalCount pageInfo {{ hasNextPage hasPreviousPage }}
                    edges {{ cursor node {{
                        id title publishedAt broadcastType status lengthSeconds
                        game {{ id name }}
                        creator {{ id login displayName }}
            }}  }}  }}  }}  }}",
            user_login, after
        );

        let j = client.query(q)?;

        if let Some(TwitchResponseData::user(u)) = j.data {
            if let Some(t) = u.videos {
                // let r = t.edges.iter().map(|f| f.node);
            }
        }

        // println!("{:?}", j);

        break;
    }

    Ok(())
}

pub fn get_channel_clips(client: &GQLClient, user_login: String) -> Result<(), ExitMsg> {
    // Paged query
    // Get all clips from a channel

    let after = "";
    loop {
        let q = format!(
            "{{  user(login: \"{}\") {{
                clips(
                    first: 100, after: {},
                    criteria: {{ period: ALL_TIME, sort: CREATED_AT_DESC }}
                ) {{
                    edges {{ cursor node {{
                        id slug title createdAt viewCount
                        durationSeconds videoOffsetSeconds
                        video {{ id }}
                        game {{ id name }}
                        broadcaster {{ id displayName login }}
                        curator {{ id displayName login }}
            }}  }}  }}  }}  }}",
            user_login, after
        );

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
        let q = format!(
            "{{  video(id: \"{}\") {{
                comments(contentOffsetSeconds: 0, after: {}) {{
                    edges {{ cursor node {{
                        contentOffsetSeconds
                        commenter {{ displayName }}
                        message {{ userColor fragments {{ mention {{ displayName }} text }} }}
            }}  }}  }}  }} }}",
            video_id, after
        );

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
        let q = format!(
            "{{  video(id: \"{}\") {{
                moments(
                    first:100,
                    momentRequestType: VIDEO_CHAPTER_MARKERS,
                    after: {}
                ) {{
                    edges {{ cursor node {{
                        description type
                        positionMilliseconds
                        durationMilliseconds
            }}  }}  }}  }}  }}",
            video_id, after
        );

        let j = client.query(q)?;

        break;
    }

    Ok(())
}

pub fn get_channel(client: &GQLClient, user_login: String) -> Result<(), ExitMsg> {
    // Single query
    // Get channel info

    let q = format!(
        "{{  user(login: \"{}\") {{
            id login displayName
            description createdAt
            roles {{ isAffiliate isPartner }}
            stream {{
                id title type
                viewersCount
                createdAt 
                game {{ id name }}
        }}  }}  }}",
        user_login
    );

    let j = client.query(q)?;

    Ok(())
}

pub fn get_video(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
    // Single query
    // Get video info

    let q = format!(
        "{{  video(id: \"{}\") {{
            id title publishedAt
            broadcastType lengthSeconds
            game {{ id name }}
            creator {{ id login displayName }}
        }}  }}",
        video_id
    );

    let j = client.query(q)?;

    Ok(())
}

pub fn get_clip(client: &GQLClient, clip_slug: String) -> Result<(), ExitMsg> {
    // Single query
    // Get clip info

    let q = format!(
        "{{  clip(slug: \"{}\") {{
            id slug title createdAt viewCount
            durationSeconds videoOffsetSeconds
            video {{ id }}
            game {{ id name }}
            videoQualities {{ frameRate quality sourceURL }}
            broadcaster {{ id displayName login }}
            curator {{ id displayName login }}
        }}  }}",
        clip_slug
    );

    let j = client.query(q)?;

    Ok(())
}

pub fn get_playback_access_token(client: &GQLClient, video_id: String) -> Result<(), ExitMsg> {
    // Single query
    // Get playback access token (for downloading videos)

    let q = format!(
        "{{  videoPlaybackAccessToken(
                id: \"{}\",
                params: {{
                    platform:\"web\",
                    playerBackend:\"mediaplayer\",
                    playerType:\"site\"
                }}
            ) {{ signature value }}
        }}",
        video_id
    );

    let j = client.query(q)?;

    Ok(())
}
