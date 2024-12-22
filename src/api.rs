use crate::model;
use reqwest::Error;



pub async fn get_coin_price(coin_name: &str, api_key: &str) -> Result<f64, String> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&api_key={}",
        coin_name, api_key
    );

   // Send request to CoinGecko API
    let response: model::CoinGeckoResponse = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?  // Convert reqwest::Error to String
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?; // Convert JSON parsing error to String

    // Try to get the coin price from the response
    match coin_name {
        "bitcoin" => Ok(response.bitcoin.usd), // Return the price if the coin is found
        _ => Err(format!("Coin '{}' not found", coin_name)), // Handle coin not found
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