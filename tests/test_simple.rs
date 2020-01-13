use cocko_rs::client::CockoClient;


#[cfg(test)]
mod test_simple {
    use super::*;

    #[test]
    fn call_simple_price() {
        let _simple_price = CockoClient::simple_price("01coin", "btc").unwrap();
        assert_eq!(_simple_price.contains_key("01coin"), true);
        assert_eq!(_simple_price["01coin"].contains_key("btc"), true);
    }

    /// TODO: need address for test
    // #[test]
    fn call_simple_token_price_by_id() {
        let address = "???";
        let vs_currency = "eth";
        let _simple_token_price = CockoClient::simple_token_price_by_id("ethereum", address, vs_currency).unwrap();
        assert_eq!(_simple_token_price.contains_key(address), true);
        assert_eq!(_simple_token_price[address].contains_key(vs_currency), true);
    }

    #[test]
    fn call_simple_supported_vs_currencies() {
        // TODO
        CockoClient::simple_supported_vs_currencies();
    }
}
