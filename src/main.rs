mod api;
use std::env;
use dotenv::dotenv;
use api::fetch_bitcoin_price;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = match env::var("COIN_GECKO_API_KEY") {
        Ok(key) => key, // Successfully retrieved the API key
        Err(e) => {
            eprintln!("Failed to get the API key: {}", e); // Handle error
            return; // Exit the program early
        }
    };

    match fetch_bitcoin_price(&api_key).await {
        Ok(price) => println!("Bitcoin price is USD: ${}", price),
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}
