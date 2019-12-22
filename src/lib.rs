extern crate reqwest;
#[macro_use]
// extern crate json;
extern crate serde_derive;
extern crate serde_json;

mod client;
mod coins;
mod exchange;
mod events;
mod simple;

use client::CockoClient;
use simple::CurrencyType;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_test() {
        // let client = CockoClient::ping();
        let coin_list = CockoClient::events();
    }

    #[test]
    fn call_simple_price() {
        // let client = CockoClient::simple_price("01coin", CurrencyType::BTC);
    }
}
