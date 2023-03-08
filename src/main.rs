use dotenvy::dotenv;
use std::env;

fn main() {
    println!("Hello, world!");
    dotenv().ok();
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY environment variable is not found.");
    println!("{}", api_key);
}
