use crate::model;
use reqwest::Error;
use std::collections::HashMap;



pub async fn get_coin_price(coin_name: &str, api_key: &str) -> Result<f64, String> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&api_key={}",
        coin_name, api_key
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?;

    // Check if the request was successful
    if response.status() != reqwest::StatusCode::OK {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    // Get the raw response body as a string for inspection
    let body = response.text().await.map_err(|e| format!("Failed to read body: {}", e))?;
    println!("Raw response: {}", body); // Print the raw JSON response for inspection

    // Parse the JSON response into a HashMap with coin names as keys
    let response_data: HashMap<String, model::CoinPrice> = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Dynamically check for the coin in the HashMap
    if let Some(coin_price) = response_data.get(coin_name) {
        Ok(coin_price.usd) // Return the price if found
    } else {
        Err(format!("Coin '{}' not found", coin_name)) // Handle coin not found
    }
}



pub async fn get_coin_historical_data(api_key: &str) -> Result<model::CoinGeckoMarketChart, Error> {
    let url = format!(
        "https://api.coingecko.com/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days=7&api_key={}",
        api_key
    );

    let response: model::CoinGeckoMarketChart = reqwest::get(&url).await?.json().await?;
    Ok(response)
}