// GQL Client, for making GQL calls to Twitch's backend.

use crate::twitch_api;
use crate::twitch_api::TwitchResponse;
use crate::util;

use rand::{distributions::Alphanumeric, Rng};
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use serde::Serialize;

#[derive(Serialize)]
struct GQLQuery {
    query: String,
}

pub struct GQLClient {
    client_id: String,
    device_id: String,
    url: String,
    client: Client,
}
impl GQLClient {
    pub fn new(client_id: String) -> GQLClient {
        GQLClient {
            client_id: client_id,
            device_id: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect(),
            url: String::from("https://gql.twitch.tv/gql"),
            client: Client::new(),
        }
    }

    pub fn raw_query(&self, query: String) -> Result<Response, util::ExitMsg> {
        let resp = self
            .client
            .post(&self.url)
            .header("Client-ID", &self.client_id)
            .header("X-Device-ID", &self.device_id)
            .json(&GQLQuery { query: query })
            .send()
            .map_err(|why| {
                util::ExitMsg::new(
                    util::ExitCode::CannotConnectToTwitch,
                    format!("Cannot connect to Twitch, reason: \"{}\".", why),
                )
            })?;

        if !resp.status().is_success() {
            return Err(util::ExitMsg::new(
                util::ExitCode::RequestErrorFromTwitch,
                format!(
                    "Error response from Twitch GQL: \"{}\".",
                    resp.text().unwrap()
                ),
            ));
        }

        Ok(resp)
    }

    pub fn query<T>(&self, query: String) -> Result<TwitchResponse<T>, util::ExitMsg>
    where
        T: twitch_api::TwitchData + for<'de> serde::Deserialize<'de>,
    {
        let s = self.raw_query(query.clone())?.text().unwrap();
        let j: TwitchResponse<T> = serde_json::from_str(&s).map_err(|why| util::ExitMsg::new(
            util::ExitCode::CannotParseResponseFromTwitch,
            format!(
                "Failed to parse response from Twitch, reason: \"{}\".\nQuery: `{}`\nResponse: `{}`",
                why, query, s
            ),
        ))?;

        if let Some(errors) = j.errors {
            return Err(util::ExitMsg::new(
                util::ExitCode::GQLErrorFromTwitch,
                format!(
                    "Something went wrong with the GQL request: \"{:?}\".",
                    errors
                ),
            ));
        }

        Ok(j)
    }
}
