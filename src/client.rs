use std::io::Read;
use std::concat;
use json::JsonValue;
use std::collections::HashMap;

use reqwest::Client;

// TODO
// use serde_json::{Result, Value};

const API_BASE_URL: &'static str  = "https://api.coingecko.com/api/v3";

pub struct CockoClient {

}

// TODO: Define error types
pub enum ErrorType {
    UNDEFINED,
}

impl CockoClient {
    pub fn ping() -> Result<JsonValue, ErrorType> {
        const PING_URL: &'static str = "/ping";
        let api = format!("{}{}", API_BASE_URL, PING_URL);

        let resp = reqwest::get(&api);
        let mut res = match resp {
            Ok(res) => res,
            Err(Error) => return Err(ErrorType::UNDEFINED)
        };

        let mut body = String::new();
        res.read_to_string(&mut body);
        let parsed = json::parse(&body).unwrap();
        println!("Body 2: {:?}", parsed);

        return Ok(parsed)
    }

    /// Call 'simple/price' API
    ///
    ///
    pub fn simple_price() -> Result<JsonValue, ErrorType> {
        const API_SIMPLE_PRICE: &'static str = "/simple/price";
        let api = format!("{}{}", API_BASE_URL, API_SIMPLE_PRICE);
        let resp = reqwest::Client::new().get(&api)
                    .query(&[("ids", "01coin"),("vs_currencies", "btc")])
                    .send();
        
        let mut res = match resp {
            Ok(res) => res,
            Err(Error) => return Err(ErrorType::UNDEFINED)
        };

        let mut body = String::new();
        res.read_to_string(&mut body);
        let parsed = json::parse(&body).unwrap();
        println!("Body 2: {:?}", parsed);
        
        return Ok(parsed)
    }

    /// Call 'coins/list' API
    ///
    ///
    pub fn coin_list() -> String {
        const PING_URL: &'static str = "/coins/list";
        let api = format!("{}{}", API_BASE_URL, PING_URL);

        let resp = reqwest::get(&api);
        let mut res = match resp {
            Ok(res) => res,
            Err(Error) => return Err(ErrorType::UNDEFINED)
        };

        let mut body = String::new();
        res.read_to_string(&mut body);
        let parsed = json::parse(&body).unwrap();
        println!("Body 2: {:?}", parsed);

        return Ok(parsed)
    }

    /// TODO:
    fn wrap_request() {

    }
}
