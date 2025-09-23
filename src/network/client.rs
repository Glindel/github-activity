use reqwest::header::USER_AGENT;
use reqwest::{Error, Response};
use crate::network::requests:: {Request, Method};

pub struct NetworkClient {
    hostname: String,
    client: reqwest::Client,
}

impl NetworkClient {
    pub fn new() -> Self {
        NetworkClient {
            hostname: "https://api.github.com".to_string(),
            client: reqwest::Client::new()
        }
    }

    pub async fn start_request(&self, request: &impl Request) -> Result<Response, Error> {
        match request.get_method() {
            Method::Get => self.get_request(request).await,
        }
    }

    async fn get_request(&self, request: &impl Request) -> Result<Response, Error> {
        self.client
            .get(self.url_request(request))
            .header(USER_AGENT, request.get_user_agent())
            .send()
            .await
    }

    fn url_request(&self, request: &impl Request) -> String {
        format!("{}{}", self.hostname, request.get_path())
    }
}