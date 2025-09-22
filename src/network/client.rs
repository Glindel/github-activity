use crate::network::requests:: {Request, Method};

struct NetworkClient {
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

    pub async fn start_request(request: impl Request) {

    }
}