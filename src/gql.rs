// GQL Client, for making GQL calls to Twitch's backend.

use crate::twitch;
use crate::twitch_api;
use crate::util;

use reqwest::blocking::Client;
use reqwest::blocking::Response;
use serde::Serialize;

#[derive(Serialize)]
struct GQLQuery {
    query: String,
}

pub struct GQLClient {
    client_id: String,
    url: String,
    client: Client,
}
impl GQLClient {
    pub fn new(client_id: String) -> GQLClient {
        GQLClient {
            client_id: client_id,
            url: String::from("https://gql.twitch.tv/gql"),
            client: Client::new(),
        }
    }

    pub fn raw_query(&self, query: String) -> Result<Response, util::ExitMsg> {
        let resp = self
            .client
            .post(&self.url)
            .header("Client-ID", &self.client_id)
            .json(&GQLQuery { query: query })
            .send()
            .map_err(|why| util::ExitMsg {
                code: util::ExitCode::CannotConnectToTwitch,
                msg: format!("Cannot connect to Twitch, reason: \"{}\".", why),
            })?;

        if !resp.status().is_success() {
            return Err(util::ExitMsg {
                code: util::ExitCode::RequestErrorFromTwitch,
                msg: format!(
                    "Error response from Twitch GQL: \"{}\".",
                    resp.text().unwrap()
                ),
            });
        }

        Ok(resp)
    }

    pub fn query(&self, query: String) -> Result<twitch_api::TwitchResponse, util::ExitMsg> {
        self.raw_query(query)?.json().map_err(|why| util::ExitMsg {
            code: util::ExitCode::CannotParseResponseFromTwitch,
            msg: format!("Failed to parse response from Twitch, reason: \"{}\".", why),
        })
    }
}
