// GQL Client, for making GQL calls to Twitch's backend.

use crate::util;

use reqwest::blocking::Client;
use reqwest::blocking::Response;

trait GQLQuery {
    fn query(query: String) -> Result<(), util::ExitMsg>;
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

    pub fn query(&self, query: String) -> Result<Response, util::ExitMsg> {
        let resp = self.client
            .post(&self.url)
            .header("Client-ID", &self.client_id)
            .body(format!("{{\"query\":\"{}\"}}", query))
            .header("Content-Type", "application/json")
            .send()
            .map_err(|why| util::ExitMsg {
                code: util::ExitCode::CannotConnectToTwitch,
                msg: format!(
                    "Cannot connect to Twitch, reason: \"{}\".",
                    why
                ),
            })?;
        
        if !resp.status().is_success() {
            return Err(util::ExitMsg {
                code: util::ExitCode::CannotConnectToTwitch,
                msg: format!(
                    "Error response from Twitch GQL: \"{}\".",
                    resp.text().unwrap()
                ),
            })
        }
        
        Ok(resp)
    }
}