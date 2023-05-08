// Twitch library for making specific queries given a GQLClient

use serde::{Deserialize, Serialize};

use crate::gql;

#[derive(Debug, Serialize, Deserialize)]
struct TwitchResponseErrorLocation {
    line: usize,
    column: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitchResponseError {
    message: String,
    locations: Vec<TwitchResponseErrorLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitchResponseExtensions {
    #[serde(rename = "durationMilliseconds")]
    duration_milliseconds: usize,
    #[serde(rename = "requestID")]
    request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchResponse {
    errors: Option<Vec<TwitchResponseError>>,
    extensions: TwitchResponseExtensions,
}

// Paged queries
// Get all videos from a channel
const QUERY_GET_CHANNEL_VIDEOS: &str = "
{{  user(login: \\\"{channel_id}\\\") {{
    videos( first: {first}, sort: {sort}, after: {after} ) {{
        totalCount
        edges {{ cursor
            node {{
                id title publishedAt broadcastType status lengthSeconds
                game {{ id name }}
                creator {{ id login displayName }}
}}  }}  }}  }}  }}
";
// Get all clips from a channel
const QUERY_GET_CHANNEL_CLIPS: &str = "
{{  user(login: \\\"{channel_id}\\\") {{
    clips(
        first: {first}, after: {after},
        criteria: {{ period: ALL_TIME, sort: CREATED_AT_DESC }}
    ) {{
        edges {{ cursor
            node {{ id slug title createdAt viewCount
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
        moments(first:100, momentRequestType: VIDEO_CHAPTER_MARKERS, after: {after}) {{
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
        id title type viewersCount createdAt game {{ id name }}
}}  }}  }}
";
// Get video
const QUERY_GET_VIDEO: &str = "
{{  video(id: \\\"{video_id}\\\") {{
    id title publishedAt
    broadcastType lengthSeconds
    game {{ id name }} creator {{ id login displayName }}
}}  }}
";
// Get clip
const QUERY_GET_CLIP: &str = "
{{  clip(slug: \\\"{clip_slug}\\\") {{
    id slug title createdAt viewCount durationSeconds videoOffsetSeconds
    game {{ id name }} video {{ id }}
    videoQualities {{ frameRate quality sourceURL }}
    broadcaster {{ id displayName login }}
    curator {{ id displayName login }}
}}  }}
";
// Get playback access token (for download videos)
const QUERY_GET_PLAYBACK_ACCESS_TOKEN: &str = "
{{  videoPlaybackAccessToken(
        id: \\\"{video_id}\\\",
        params: {{ platform:\\\"web\\\", playerBackend:\\\"mediaplayer\\\", playerType:\\\"site\\\" }}
    ) {{ signature value }}
}}
";
