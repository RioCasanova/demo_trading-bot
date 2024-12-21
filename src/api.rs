use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct CoinGeckoResponse {
    bitcoin: Price,
}

#[derive(Deserialize)]
struct Price {
    usd: f64,
}

pub async fn fetch_bitcoin_price(api_key: &str) -> Result<f64, Error> {
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&api_key={}", api_key);
    
    let response: CoinGeckoResponse = reqwest::get(&url).await?.json().await?;
    Ok(response.bitcoin.usd)
}
