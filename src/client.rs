use std::io::Read;
use std::collections::HashMap;

use serde::{Deserialize};



use crate::coins::{
    CoinsData, 
    CoinsHistoryData, 
    CoinsListItem, 
    CoinsMarketChartData, 
    CoinsMarketItem, 
    CoinsTickerData
};
use crate::events::{
    EventCountriesParam,
    EventResponseParam,
    EventTypesParam
};
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

        let _ping_data = match res.json::<PingData>() {
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
    pub fn coins_list() -> Result<Vec<CoinsListItem>, serde_json::Error> {
        const COINS_LIST: &str = "/coins/list";
        let api = format!("{}{}", API_BASE_URL, COINS_LIST);

        let mut res = reqwest::get(&api).expect("Error on request");

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_list_map: Vec<CoinsListItem> = serde_json::from_str(&body)?;
        
        Ok(coin_list_map)
    }

    /// Call 'coins/market' API
    ///
    ///
    pub fn coins_markets(vs_currency: &str) -> Result<Vec<CoinsMarketItem>, serde_json::Error> {
        const COINS_MARKETS: &str = "/coins/markets";
        let api = format!("{}{}", API_BASE_URL, COINS_MARKETS);

        let mut res = reqwest::Client::new().get(&api).query(&[
            ("vs_currency", vs_currency)
        ])
        .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_list_map: Vec<CoinsMarketItem> = serde_json::from_str(&body)?;
        
        Ok(coin_list_map)
    }

    /// Call 'coins/{id}' API
    ///
    ///
    pub fn coins_with_id(id: &str) -> Result<CoinsData, serde_json::Error> {
        const COINS: &str = "/coins";
        let api = format!("{}{}/{}", API_BASE_URL, COINS, id);

        let mut res = reqwest::get(&api).expect("Error on request");

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_data: CoinsData = serde_json::from_str(&body)?;
        
        Ok(coin_data)
    }

    /// Call 'coins/{id}/tickers' API
    ///
    ///
    pub fn coins_with_id_tickers(id: &str) -> Result<CoinsTickerData, serde_json::Error> {
        const COINS: &str = "/coins";
        const TICKER: &str = "/tickers";
        let api = format!("{}{}/{}{}", API_BASE_URL, COINS, id, TICKER);
        let mut res = reqwest::get(&api).expect("Error on request");

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_ticker_data: CoinsTickerData = serde_json::from_str(&body)?;
        
        Ok(coin_ticker_data)
    }

    /// Call 'coins/{id}/history' API
    ///
    ///
    pub fn coins_with_id_history(id: &str, date: &str) -> Result<CoinsHistoryData, serde_json::Error> {
        const COINS: &str = "/coins";
        const HISTORY: &str = "/history";
        let api = format!("{}{}/{}{}", API_BASE_URL, COINS, id, HISTORY);
        let mut res = reqwest::Client::new().get(&api).query(&[
            ("date", date)
        ])
        .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_history_data: CoinsHistoryData = serde_json::from_str(&body)?;
        
        Ok(coin_history_data)
    }

    /// Call 'coins/{id}/market_chart' API
    ///
    ///
    pub fn coins_with_id_market_chart(id: &str, vs_currency: &str, days: &str) -> Result<CoinsMarketChartData, serde_json::Error> {
        const COINS: &str = "/coins";
        const MARKET_CHART: &str = "/market_chart";
        let api = format!("{}{}/{}{}", API_BASE_URL, COINS, id, MARKET_CHART);
        let mut res = reqwest::Client::new().get(&api).query(&[
            ("vs_currency", vs_currency),
            ("days", days),
        ])
        .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_market_chart_data: CoinsMarketChartData = serde_json::from_str(&body)?;
        
        Ok(coin_market_chart_data)
    }

    /// Call 'coins/{id}/contract/{contract_address}' API
    /// TODO: Beta status
    ///
    fn coins_with_id_with_contract_address(id: &str, contract_address: &str) -> Result<CoinsData, serde_json::Error> {
        const COINS: &str = "/coins";
        const MARKET_CHART: &str = "/contract";
        let api = format!("{}{}/{}{}/{}", API_BASE_URL, COINS, id, MARKET_CHART, contract_address);
        let mut res = reqwest::Client::new().get(&api).send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body);
        let coin_market_chart_data: CoinsData = serde_json::from_str(&body)?;
        
        Ok(coin_market_chart_data)
    }


    /// Call '/exchanges'
    /// TODO: Beta status
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
    /// TODO: Beta status
    ///
    fn exchange_list() -> Result<Vec<ExchangesMarketItem>, serde_json::Error> {
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

        Ok(events)
    }

    /// Call '/events/countries'
    ///
    ///
    pub fn events_countries() -> Result<EventCountriesParam, serde_json::Error> {
        const EVENTS: &str = "/events";
        const COUNTRIES: &str = "/countries";
        let api = format!("{}{}{}", API_BASE_URL, EVENTS, COUNTRIES);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let events_countries: EventCountriesParam = serde_json::from_str(&body).unwrap();

        Ok(events_countries)
    }

    /// Call '/events/types'
    ///
    ///
    pub fn events_types() -> Result<EventTypesParam, serde_json::Error> {
        const EVENTS: &str = "/events";
        const TYPES: &str = "/types";
        let api = format!("{}{}{}", API_BASE_URL, EVENTS, TYPES);
        let mut res = reqwest::get(&api).expect("Error on request");
        let mut body = String::new();

        res.read_to_string(&mut body);
        let events_countries: EventTypesParam = serde_json::from_str(&body).unwrap();
        
        Ok(events_countries)
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
