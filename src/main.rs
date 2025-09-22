mod network;
mod models;

use crate::network::client::NetworkClient;
use crate::network::requests::EventRequest;

use tokio::runtime::Runtime;
use reqwest::{ Error, Response, StatusCode };
use std::env;

fn main() {
    let Some(user) = env::args().nth(1) else {
        println!("No user provided");
        return;
    };

    let client = NetworkClient::new();

    let runtime = Runtime::new().unwrap();
    match runtime.block_on(fetch_user_events(&client, user.as_str())) {
        Ok(events) => { println!("{:?}", events) },
        Err(e) => { println!("{:?}", e); }
    }

}

async fn fetch_user_events(client: &NetworkClient, user: &str) -> Result<Response, Error> {
    let request = EventRequest::new(user);
    let response = client.start_request(&request).await.expect("Unexpected error");

    match response.status() {
        StatusCode::OK => response.json() {

        }
    }
}

async fn
