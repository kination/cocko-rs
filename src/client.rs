use std::io::Read;
use std::collections::HashMap;

use serde::{Deserialize};
use serde_json::{Value};

use crate::simple::CurrencyType;
use crate::coins::CoinListItem;
use crate::events::{EventResponseParam};
use crate::finance::FinancePlatformItem;
use crate::exchange::{ExchangesItem, ExchangesMarketItem, ExchangeRates};

const API_BASE_URL: &str  = "https://api.coingecko.com/api/v3";

pub struct CockoClient {

}

#[derive(Debug, Deserialize)]
struct PingData {
    gecko_says: String
}

// TODO: Define error types
pub enum ErrorType {
    UNDEFINED,
}

#[allow(dead_code)]
impl CockoClient {
    pub fn ping() -> Result<String, ErrorType> {
        const PING_URL: &str = "/ping";
        let api = format!("{}{}", API_BASE_URL, PING_URL);

        let resp = reqwest::get(&api);
        let mut res = match resp {
            Ok(res) => res,
            Err(_error) => return Err(ErrorType::UNDEFINED)
        };

        let ping_data = match res.json::<PingData>() {
            Ok(ping_data) => ping_data,
            Err(_error) => return Err(ErrorType::UNDEFINED)
        };

        Ok("parsed".to_string())
    }

    /// Call 'simple/price' API
    ///
    ///
    pub fn simple_price(_ids: &str, _vs_currencies: &str) -> Result<HashMap<String, HashMap<String, f64>>, serde_json::Error> {
        const API_SIMPLE_PRICE: &str = "/simple/price";
        let api = format!("{}{}", API_BASE_URL, API_SIMPLE_PRICE);
        let mut res = reqwest::Client::new().get(&api)
                    .query(&[("ids", _ids),("vs_currencies", _vs_currencies)])
                    .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body);
        let price: HashMap<String, HashMap<String, f64>> = serde_json::from_str(&body)?;
        
        Ok(price)
    }

    /// Call 'simple/token_price/{id}' API
    ///
    ///
    pub fn simple_token_price_by_id(token_id: &str, contract_addresses: &str, vs_currencies: &str) -> Result<HashMap<String, HashMap<String, f64>>, serde_json::Error> {
        const API_SIMPLE_TOKEN_PRICE: &str = "/simple/token_price";
        let _api = format!("{}{}/{}", API_BASE_URL, API_SIMPLE_TOKEN_PRICE, token_id);

        let mut res = reqwest::Client::new().get(&_api)
                    .query(&[
                        ("contract_addresses", contract_addresses),
                        ("vs_currencies", vs_currencies)
                    ])
                    .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body);
        
        let price: HashMap<String, HashMap<String, f64>> = serde_json::from_str(&body)?;
        
        Ok(price)
    }

     /// Call 'simple/supported_vs_currencies' API
    ///
    ///
    pub fn simple_supported_vs_currencies() {
        const API_SIMPLE_SUPPORTED_CURRENCIES: &str = "/simple/supported_vs_currencies";
        let _api = format!("{}{}", API_BASE_URL, API_SIMPLE_SUPPORTED_CURRENCIES);
        
        // TODO:
    }

    /// Call 'coins/list' API
    ///
    ///
    pub fn coins_list() -> Result<Vec<CoinListItem>, serde_json::Error> {
        const COINS_LIST: &str = "/coins/list";
        let api = format!("{}{}", API_BASE_URL, COINS_LIST);

        let mut res = reqwest::get(&api).expect("Error on request");

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_list_map: Vec<CoinListItem> = serde_json::from_str(&body)?;
        
        Ok(coin_list_map)
    }

    /// Call '/exchanges'
    ///
    ///
    pub fn exchanges () -> Result<Vec<ExchangesItem>, serde_json::Error> {
        const EXCHANGES: &str = "/exchanges";
        let api = format!("{}{}", API_BASE_URL, EXCHANGES);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let exchanges: Vec<ExchangesItem> = serde_json::from_str(&body).unwrap();

        Ok(exchanges)
    }

    /// Call '/exchanges/list'
    ///
    ///
    pub fn exchange_list() -> Result<Vec<ExchangesMarketItem>, serde_json::Error> {
        const EXCHANGES_LIST: &str = "/exchanges/list";
        let api = format!("{}{}", API_BASE_URL, EXCHANGES_LIST);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let exchange_list: Vec<ExchangesMarketItem> = serde_json::from_str(&body).unwrap();

        Ok(exchange_list)
    }

    /// Call '/exchange_rates'
    ///
    ///
    pub fn exchange_rates() -> Result<ExchangeRates, serde_json::Error> {
        const EXCHANGE_RATES: &str = "/exchange_rates";
        let api = format!("{}{}", API_BASE_URL, EXCHANGE_RATES);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let exchange_list: ExchangeRates = serde_json::from_str(&body).unwrap();

        Ok(exchange_list)
    }

    /// Call '/events'
    ///
    ///
    pub fn events() -> Result<EventResponseParam, serde_json::Error> {
        const EVENTS: &str = "/events";
        let api = format!("{}{}", API_BASE_URL, EVENTS);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let events: EventResponseParam = serde_json::from_str(&body).unwrap();
        println!("Event item 1: {:?}", events.data[0]);
        Ok(events)
    }

    /// Call '/finance_platforms'
    ///
    ///
    pub fn finance_platforms() -> Result<Vec<FinancePlatformItem>, serde_json::Error> {
        const FINANCE_PLATFORMS: &str = "/finance_platforms";
        let api = format!("{}{}", API_BASE_URL, FINANCE_PLATFORMS);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let events: Vec<FinancePlatformItem> = serde_json::from_str(&body).unwrap();
        println!("Event item 1: {:?}", events[0]);
        Ok(events)
    }

    /// TODO:
    fn wrap_request() {

    }
}
