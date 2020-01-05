use cocko_rs::client::CockoClient;

use cocko_rs::exchange::{ExchangeRates};

#[cfg(test)]
mod test_simple {
    use super::*;

    #[test]
    fn call_exchange_rates() {
        let _exchange_rates: ExchangeRates = CockoClient::exchange_rates().unwrap();
        println!("_exchange_rates: {:?}", _exchange_rates.rates["btc"].name);
    }
}