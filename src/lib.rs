use dotenvy::dotenv;
use std::env;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};

pub fn api_key() -> String {
    dotenv().ok();
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY environment variable is not found.");
    return api_key;
}

pub fn call_chat_api(api_key: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let v = "Bearer ".to_owned() + api_key;
    headers.insert("Authorization", HeaderValue::from_str(&v).unwrap());

    let client = Client::new();
    let response = client
	.get("https://api.openai.com/v1/models")
	.headers(headers)
	.send()?;
    Ok(response)
}

