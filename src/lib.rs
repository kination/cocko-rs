extern crate reqwest;
#[macro_use]
// extern crate json;
extern crate serde_derive;
extern crate serde_json;

mod client;
mod simple;
mod coins;

use client::CockoClient;
use simple::CurrencyType;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_ping() {
        // let client = CockoClient::ping();
    }

    #[test]
    fn call_simple_price() {
        // let client = CockoClient::simple_price("01coin", CurrencyType::BTC);
    }

    #[test]
    fn call_coin_list() {
        let coin_list = CockoClient::coin_list();
    }
}
