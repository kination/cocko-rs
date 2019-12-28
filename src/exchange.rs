use std::collections::HashMap;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExchangesItem {
    id: String,
    name: String,
    year_established: Option<i32>,
    country: Option<String>,
    description: Option<String>,
    url: String,
    image: String,
    has_trading_incentive: Option<bool>,
    trust_score: i32,
    trust_score_rank: i32,
    trade_volume_24h_btc: f64,
    trade_volume_24h_btc_normalized: f64
}

#[derive(Debug, Deserialize)]
pub struct ExchangesMarketItem {
    id: String,
    name: String
}

#[derive(Debug, Deserialize)]
pub struct ExchangeRates {
    rates: HashMap<String, ExchangeRatesItem>
}

#[derive(Debug, Deserialize)]
pub struct ExchangeRatesItem {
    name: String,
    unit: String,
    value: f64,
    r#type: String
}
