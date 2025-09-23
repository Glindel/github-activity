mod network;
mod models;

use crate::network::client::NetworkClient;
use crate::network::requests::EventRequest;
use crate::models::event::Event;

use tokio::runtime::Runtime;
use reqwest::{ Error };
use std::env;

fn main() {
    let Some(user) = env::args().nth(1) else {
        println!("No user provided");
        return;
    };

    let client = NetworkClient::new();

    let runtime = Runtime::new().unwrap();

    println!("-----------------------------------");
    println!("Event Checker for User: {}", user);
    println!("-----------------------------------");


    match runtime.block_on(fetch_user_events(&client, user.as_str())) {
        Ok(events) => { present_event_data(events) }
        Err(e) => {println!("Unexpected error: {:?}", e);}
    }

}

fn present_event_data(events: Vec<Event>) {
    if events.is_empty() {
        println!("No events found for this user...");
        return;
    }

    for event in events {
        println!("{}", event.event_message());
    }
}

async fn fetch_user_events(client: &NetworkClient, user: &str) -> Result<Vec<Event>, Error> {
    let request = EventRequest::new(user);
    let response = client.start_request(&request).await.expect("Unexpected error");
    response.json::<Vec<Event>>().await
}
