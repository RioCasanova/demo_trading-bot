pub const BASE_URL: &str = "https://api.coingecko.com/api/v3/";


// returns the cost of the coin at the moment, usd_market_cap, usd_24h_vol, usd_24h_change, last_updated_at
/*
{
  "bitcoin": {
    "usd": 67187.3358936566,
    "usd_market_cap": 1317802988326.25,
    "usd_24h_vol": 31260929299.5248,
    "usd_24h_change": 3.63727894677354,
    "last_updated_at": 1711356300
  }
}
*/
pub fn coin_price_endpoint(coin_id: &str, key: &str) -> String {
    format!("{}simple/price?ids={}&vs_currencies=usd&api_key={}", BASE_URL, coin_id, key)
}


// Lots of information per coin added
/*
{
  "id": "bitcoin",
  "symbol": "btc",
  "name": "Bitcoin",
  "web_slug": "bitcoin",
  "asset_platform_id": null,
  "platforms": {
    "": ""
  },
  "detail_platforms": {
    "": {
      "decimal_place": null,
      "contract_address": ""
    }
  },
  "block_time_in_minutes": 10,
  "hashing_algorithm": "SHA-256",
  "categories": [
    "FTX Holdings",
    "Cryptocurrency",
    "Proof of Work (PoW)",
    "Layer 1 (L1)"
  ],
  "preview_listing": false,
  "public_notice": null,
  "additional_notices": [],
  "localization": {
    "en": "Bitcoin",
    "de": "Bitcoin"
  },
*/
pub fn coin_info_endpoint(coin_id: &str) -> String {
    format!("{}coins/{}", BASE_URL, coin_id)
}


// Market data logged about particular coin
/*
[
  {
    "id": "bitcoin",
    "symbol": "btc",
    "name": "Bitcoin",
    "image": "https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1696501400",
    "current_price": 70187,
    "market_cap": 1381651251183,
    "market_cap_rank": 1,
    "fully_diluted_valuation": 1474623675796,
    "total_volume": 20154184933,
    "high_24h": 70215,
    "low_24h": 68060,
    "price_change_24h": 2126.88,
    "price_change_percentage_24h": 3.12502,
    "market_cap_change_24h": 44287678051,
    "market_cap_change_percentage_24h": 3.31157,
    "circulating_supply": 19675987,
    "total_supply": 21000000,
    "max_supply": 21000000,
    "ath": 73738,
    "ath_change_percentage": -4.77063,
    "ath_date": "2024-03-14T07:10:36.635Z",
    "atl": 67.81,
    "atl_change_percentage": 103455.83335,
    "atl_date": "2013-07-06T00:00:00.000Z",
    "roi": null,
    "last_updated": "2024-04-07T16:49:31.736Z"
  }
]
*/

pub fn coin_market_data_endpoint(coin_id: &str, currency: &str) -> String {
    format!("{}coins/{}/markets?vs_currency={}", BASE_URL, coin_id, currency)
}




/*
{
  "prices": [
    [
      1711843200000,
      69702.3087473573
    ],
    [
      1711929600000,
      71246.9514406015
    ],
    [
      1711983682000,
      68887.7495158568
    ]
  ],
  "market_caps": [
    [
      1711843200000,
      1370247487960.09
    ],
    [
      1711929600000,
      1401370211582.37
    ],
    [
      1711983682000,
      1355701979725.16
    ]
  ],
  "total_volumes": [
    [
      1711843200000,
      16408802301.8374
    ],
    [
      1711929600000,
      19723005998.215
    ],
    [
      1711983682000,
      30137418199.6431
    ]
  ]
}
*/

pub fn coin_market_chart_endpoint(coin_id: &str, currency: &str, days: u32) -> String {
    format!("{}coins/{}/market_chart?vs_currency={}&days={}", BASE_URL, coin_id, currency, days)
}


#[warn(dead_code)] // not needed 
pub fn coin_history_endpoint(coin_id: &str, date: &str) -> String {
    format!("{}coins/{}/history?date={}", BASE_URL, coin_id, date)
}

#[warn(dead_code)] // not needed 
pub fn coin_market_chart_range_endpoint(coin_id: &str, currency: &str, from: u32, to: u32) -> String {
    format!("{}coins/{}/market_chart/range?vs_currency={}&from={}&to={}", BASE_URL, coin_id, currency, from, to)
}

#[warn(dead_code)] // not needed 
pub fn coins_list_endpoint() -> String {
    format!("{}coins/list", BASE_URL)
}

#[warn(dead_code)] // not needed 
pub fn coin_status_updates_endpoint(coin_id: &str) -> String {
    format!("{}coins/{}/status_updates", BASE_URL, coin_id)
}

#[warn(dead_code)] // not needed 
pub fn global_market_data_endpoint() -> String {
    format!("{}global", BASE_URL)
}

#[warn(dead_code)] // not needed 
pub fn global_market_data_with_currency_endpoint(currency: &str) -> String {
    format!("{}global?vs_currency={}", BASE_URL, currency)
}

#[warn(dead_code)] // not needed 
pub fn coin_tickers_endpoint(coin_id: &str) -> String {
    format!("{}coins/{}/tickers", BASE_URL, coin_id)
}

#[warn(dead_code)] // not needed 
pub fn coin_ohlcv_endpoint(coin_id: &str, currency: &str, days: u32) -> String {
    format!("{}coins/{}/ohlcv/{}-day?vs_currency={}", BASE_URL, coin_id, days, currency)
}

#[warn(dead_code)] // not needed 
pub fn coin_category_endpoint(category_id: &str) -> String {
    format!("{}coins/categories/{}", BASE_URL, category_id)
}