use std::io::Read;
use std::concat;
use json::JsonValue;
// use serde_json::{Result, Value};

const API_BASE_URL: &'static str  = "https://api.coingecko.com/api/v3";

pub struct Client {

}

// TODO: Define error types
pub enum ErrorType {
    UNDEFINED,
}

impl Client {
    
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

    fn coin_list() -> String {
        return String::from("OK")
    }
}
