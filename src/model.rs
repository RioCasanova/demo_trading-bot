use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct CoinPrice {
    pub usd: f64,
    
}



#[derive(Deserialize, Clone, Debug)]
pub struct CoinGeckoMarketData {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: f64,
    pub market_cap: u64,
    pub market_cap_rank: u32,
    pub fully_diluted_valuation: Option<u64>,
    pub total_volume: u64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub price_change_24h: f64,
    pub price_change_percentage_24h: f64,
    pub market_cap_change_24h: f64,
    pub market_cap_change_percentage_24h: f64,
    pub circulating_supply: f64,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: f64,
    pub ath_change_percentage: f64,
    pub ath_date: String,
    pub atl: f64,
    pub atl_change_percentage: f64,
    pub atl_date: String,
    pub roi: Option<String>, // null in the example, adjust type if necessary
    pub last_updated: String,
}

