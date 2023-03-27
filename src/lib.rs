use dotenvy::dotenv;
use std::env;
use reqwest::blocking::Client;

pub fn api_key() -> String {
    dotenv().ok();
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY environment variable is not found.");
    return api_key;
}

pub fn call_chat_api(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client.get("https://google.com").send()?;
    println!("{}", api_key);
    println!("{:#?}", resp.text()?);
    Ok(())
}

