use dotenvy::dotenv;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Models {
    object: String,
    pub data: Vec<Model>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub id: String,
}

pub fn api_key() -> String {
    dotenv().ok();
    env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY environment variable is not found.")
}

pub fn call_chat_api(api_key: &str, path: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let v = "Bearer ".to_owned() + api_key;
    headers.insert("Authorization", HeaderValue::from_str(&v).unwrap());

    let client = Client::new();
    let response = client
        .get(format!("{}{}", "https://api.openai.com", path))
        .headers(headers)
        .send()?;
    Ok(response)
}

pub fn get_models() -> Models {
    call_chat_api(&api_key(), "/v1/models")
        .unwrap()
        .json()
        .unwrap()
}
