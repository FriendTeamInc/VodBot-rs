// Twitch library for making specific queries given a GQLClient

use crate::{gql, util};

pub fn get_channel_videos(
    client: &gql::GQLClient,
    user_login: String,
) -> Result<(), util::ExitMsg> {
    // Paged query
    // Get all videos from a channel

    let after = "";
    loop {
        let q = format!(
            "
        {{  user(login: \"{}\") {{
            videos( first: 100, sort: TIME, after: \"{}\" ) {{
                totalCount edges {{ cursor node {{
                    id title publishedAt broadcastType status lengthSeconds
                    game {{ id name }}
                    creator {{ id login displayName }}
        }}  }}  }}  }}  }}
        ",
            user_login, after
        );

        let j = client.query(q)?;

        break;
    }
    
    Ok(())
}

// Get all clips from a channel
const QUERY_GET_CHANNEL_CLIPS: &str = "
{{  user(login: \\\"{channel_id}\\\") {{
    clips(
        first: 100, after: {after},
        criteria: {{ period: ALL_TIME, sort: CREATED_AT_DESC }}
    ) {{
        edges {{ cursor node {{
            id slug title createdAt viewCount
            durationSeconds videoOffsetSeconds
            video {{ id }}
            game {{ id name }}
            broadcaster {{ id displayName login }}
            curator {{ id displayName login }}
}}  }}  }}  }}  }}
";
// Get all "comments" (chat messages) from a video
const QUERY_GET_VIDEO_COMMENTS: &str = "
{{  video(id: \\\"{video_id}\\\") {{
        comments(contentOffsetSeconds: 0, after: {after}) {{
            edges {{ cursor node {{
                contentOffsetSeconds
                commenter {{ displayName }}
                message {{ userColor fragments {{ mention {{ displayName }} text }} }}
}}  }}  }}  }} }}
";
// Get all video chapters (things like game changes) from a video
const QUERY_GET_VIDEO_CHAPTERS: &str = "
{{  video(id: {id}) {{
        moments(
            first:100,
            momentRequestType: VIDEO_CHAPTER_MARKERS,
            after: {after}
        ) {{
            edges {{ cursor node {{
                description type positionMilliseconds durationMilliseconds
}}  }}  }}  }}  }}
";

// Individual queries
// Get channel
const QUERY_GET_CHANNEL: &str = "
{{  user(login: \\\"{channel_id}\\\") {{
    id login displayName
    description createdAt
    roles {{ isAffiliate isPartner }}
    stream {{
        id title type viewersCount
        createdAt game {{ id name }}
}}  }}  }}
";
// Get video
const QUERY_GET_VIDEO: &str = "
{{  video(id: \\\"{video_id}\\\") {{
    id title publishedAt
    broadcastType lengthSeconds
    game {{ id name }}
    creator {{ id login displayName }}
}}  }}
";
// Get clip
const QUERY_GET_CLIP: &str = "
{{  clip(slug: \\\"{clip_slug}\\\") {{
    id slug title createdAt viewCount
    durationSeconds videoOffsetSeconds
    video {{ id }}
    game {{ id name }}
    videoQualities {{ frameRate quality sourceURL }}
    broadcaster {{ id displayName login }}
    curator {{ id displayName login }}
}}  }}
";
// Get playback access token (for download videos)
const QUERY_GET_PLAYBACK_ACCESS_TOKEN: &str = "
{{  videoPlaybackAccessToken(
        id: \\\"{video_id}\\\",
        params: {{
            platform:\\\"web\\\",
            playerBackend:\\\"mediaplayer\\\",
            playerType:\\\"site\\\"
        }}
    ) {{ signature value }}
}}
";
