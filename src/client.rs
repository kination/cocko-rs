use std::io::Read;
use std::concat;

use serde::{Deserialize};
use serde_json::{Value};
use std::collections::HashMap;

use reqwest::Client;

use crate::simple::CurrencyType;
use crate::coins::CoinListItem;
use crate::events::{EventResponseParam};
use crate::exchange::{ExchangesItem, ExchangesMarketItem};


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
    pub fn coins_list() -> Result<Vec<CoinListItem>, serde_json::Error> {
        const COINS_LIST: &'static str = "/coins/list";
        let api = format!("{}{}", API_BASE_URL, COINS_LIST);

        let mut res = reqwest::get(&api).expect("Error on request");

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_list_map: Vec<CoinListItem> = serde_json::from_str(&body)?;
        
        return Ok(coin_list_map)
    }

    /// Call '/exchanges'
    ///
    ///
    pub fn exchanges () -> Result<Vec<ExchangesItem>, serde_json::Error> {
        const EXCHANGES: &'static str = "/exchanges";
        let api = format!("{}{}", API_BASE_URL, EXCHANGES);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let exchanges: Vec<ExchangesItem> = serde_json::from_str(&body).unwrap();

        return Ok(exchanges)
    }

    /// Call '/exchanges/list'
    ///
    ///
    pub fn exchange_list() -> Result<Vec<ExchangesMarketItem>, serde_json::Error> {
        const EXCHANGES_LIST: &'static str = "/exchanges/list";
        let api = format!("{}{}", API_BASE_URL, EXCHANGES_LIST);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let exchange_list: Vec<ExchangesMarketItem> = serde_json::from_str(&body).unwrap();

        return Ok(exchange_list)
    }

    /// Call '/events'
    ///
    ///
    pub fn events() -> Result<EventResponseParam, serde_json::Error> {
        const EVENTS: &'static str = "/events";
        let api = format!("{}{}", API_BASE_URL, EVENTS);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let events: EventResponseParam = serde_json::from_str(&body).unwrap();
        println!("Event item 1: {:?}", events.data[0]);
        return Ok(events)
    }

    /// TODO:
    fn wrap_request() {

    }
}
