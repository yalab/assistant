use dotenvy::dotenv;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn api_key() -> String {
    dotenv().ok();
    env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY environment variable is not found.")
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

    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    Ok(response)
}
