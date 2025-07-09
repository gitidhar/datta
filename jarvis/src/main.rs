
use std::env;
use dotenv::dotenv;

fn main() { 

    dotenv().ok();
    println!("Hello Worlsssd");
    let api_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY not set in .env or environment");
}