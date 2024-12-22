use once_cell::sync::Lazy;
use std::env;
use dotenv::dotenv;


//  Fetches the API key and stores it for reference in memory
static API_KEY: Lazy<String> = Lazy::new(|| {
    dotenv().ok(); // Load environment variables

    env::var("COIN_GECKO_API_KEY").unwrap_or_else(|err| {
        eprintln!("Failed to get the API key: {}", err);
        std::process::exit(1); // Exit if the API key is not found
    })
});

pub fn get_api_key() -> &'static str {
    &API_KEY
}