use crate::model;
use crate::endpoint;
use std::collections::HashMap;



pub async fn get_coin_price(coin_name: &str, api_key: &str) -> Result<f64, String> {
    let url = endpoint::coin_price_endpoint(coin_name, api_key);

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



pub async fn get_coin_historical_data(
    coin_name: &str,
    api_key: &str,
) -> Result<model::CoinGeckoMarketData, String> {
    // Build the URL for the API endpoint.
    let url = endpoint::coin_market_data_endpoint(coin_name, api_key);

    // Perform the HTTP GET request.
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?;

    // Ensure the request was successful (HTTP 200 OK).
    if !response.status().is_success() {
        return Err(format!("Request failed with status code: {}", response.status()));
    }

    // Read the response body as a string.
    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    println!("Raw response: {}", body); // Print for debugging.

    // Parse the response JSON into a HashMap.
    let response_data: HashMap<String, model::CoinGeckoMarketData> = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Look for the coin data in the parsed HashMap.
    response_data
        .get(coin_name)
        .cloned() // Cloned to move ownership out of the HashMap.
        .ok_or_else(|| format!("Coin '{}' not found in API response", coin_name))
}