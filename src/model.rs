use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct CoinPrice {
    pub usd: f64,
    
}



#[derive(Deserialize)]
pub struct CoinGeckoMarketChart {
   pub prices: Vec<Vec<f64>>,
   pub market_caps: Vec<Vec<f64>>,
   pub total_volumes: Vec<Vec<f64>>,
}

