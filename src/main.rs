mod network;

use tokio::runtime::Runtime;
use reqwest;
use reqwest::Error;
use std::env;

fn main() {
    let Some(user) = env::args().nth(1) else {
        println!("No user provided");
        return;
    };

    let runtime = Runtime::new().unwrap();
    match runtime.block_on(fetch_user_events(user.as_str())) {
        Ok(events) => { println!("{:?}", events) },
        Err(e) => { println!("{:?}", e); }
    }

}

async fn fetch_user_events(user: &str) -> Result<String,Error> {
    reqwest::get(format!("https://api.github.com/users/{}/events", user)).await?.text().await
}
