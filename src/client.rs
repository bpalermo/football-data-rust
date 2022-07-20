use crate::competition::Competitions;
use reqwest::{header, Client, Error};

const DEFAULT_BASE_URL: &str = "https://api.football-data.org/v4";
const DEFAULT_AUTH_TOKEN: &str = "";

pub struct FootballDataClient {
    http_client: Client,
    base_url: String,
}

impl FootballDataClient {
    pub fn new(base_url: Option<String>, auth_token: Option<String>) -> Self {
        let mut h = header::HeaderMap::new();

        h.insert(
            "X-Auth-Token",
            header::HeaderValue::from_str(&auth_token.unwrap_or(String::from(DEFAULT_AUTH_TOKEN)))
                .unwrap(),
        );
        h.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder().default_headers(h).build().unwrap();

        Self {
            http_client: client,
            base_url: base_url.unwrap_or(String::from(DEFAULT_BASE_URL)),
        }
    }

    pub async fn get_competitions(&self) -> Result<Competitions, Error> {
        let url = format!(
            "{}{}",
            self.base_url,
            crate::competition::COMPETITIONS_ENDPOINT
        );
        let competitions: Competitions = self.http_client.get(&url).send().await?.json().await?;
        Ok(competitions)
    }
}
