use dotenvy::dotenv;
use std::env;

pub fn api_key() -> String {
    dotenv().ok();
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY environment variable is not found.");
    return api_key;
}
