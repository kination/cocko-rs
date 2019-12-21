use std::io::Read;
use std::concat;

use serde::{Deserialize};
use serde_json::{Value};
use std::collections::HashMap;

use reqwest::Client;

use crate::simple::CurrencyType;
use crate::coins::CoinListItem;


const API_BASE_URL: &'static str  = "https://api.coingecko.com/api/v3";

pub struct CockoClient {

}

#[derive(Debug, Deserialize)]
struct PingData {
    gecko_says: String
}

struct CoinData {
    
}

// TODO: Define error types
pub enum ErrorType {
    UNDEFINED,
}

impl CockoClient {
    pub fn ping() -> Result<String, ErrorType> {
        const PING_URL: &'static str = "/ping";
        let api = format!("{}{}", API_BASE_URL, PING_URL);

        let resp = reqwest::get(&api);
        let mut res = match resp {
            Ok(res) => res,
            Err(error) => return Err(ErrorType::UNDEFINED)
        };

        let ping_data = match res.json::<PingData>() {
            Ok(ping_data) => ping_data,
            Err(error) => return Err(ErrorType::UNDEFINED)
        };

        println!("Body 2: {:?}", ping_data.gecko_says);

        return Ok("parsed".to_string())
    }

    /// Call 'simple/price' API
    ///
    ///
    pub fn simple_price(ids: &str, vs_currencies: CurrencyType) 
    -> Result<String, serde_json::Error> {
        const API_SIMPLE_PRICE: &'static str = "/simple/price";
        let api = format!("{}{}", API_BASE_URL, API_SIMPLE_PRICE);
        let mut res = reqwest::Client::new().get(&api)
                    .query(&[("ids", "01coin"),("vs_currencies", "btc")])
                    .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body);
        let map: Value = serde_json::from_str(&body)?;
        println!("Body 2: {}", map["01coin"]["btc"]);
        
        return Ok("parsed".to_string())
    }

    /// Call 'simple/token_price/{id}' API
    ///
    ///
    pub fn simple_token_price_by_id(token_id: &str) {
        const API_SIMPLE_PRICE: &'static str = "/simple/token_price";
        let api = format!("{}{}/{}", API_BASE_URL, API_SIMPLE_PRICE, token_id);
        
        // TODO:
    }

    /// Call 'coins/list' API
    ///
    ///
    pub fn coin_list() -> Result<Vec<CoinListItem>, serde_json::Error> {
        const PING_URL: &'static str = "/coins/list";
        let api = format!("{}{}", API_BASE_URL, PING_URL);

        let mut res = reqwest::get(&api).expect("Error on request");

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_list_map: Vec<CoinListItem> = serde_json::from_str(&body)?;
        
        return Ok(coin_list_map)
    }

    /// TODO:
    fn wrap_request() {

    }
}
