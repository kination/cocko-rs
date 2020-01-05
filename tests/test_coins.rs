use cocko_rs::client::CockoClient;

use cocko_rs::coins::{
    CoinsData,
    CoinsHistoryData,
    CoinsListItem,
    CoinsMarketChartData,
    CoinsMarketItem,
    CoinsTickerData
};


#[cfg(test)]
mod test_simple {
    use super::*;

    #[test]
    fn call_coins_list() {
        let _coins_list: Vec<CoinsListItem> = CockoClient::coins_list().unwrap();
        assert_eq!(!_coins_list.is_empty(), true);
        assert_eq!(_coins_list[0].id, "01coin");
    }

    #[test]
    fn call_coins_market() {
        let _coins_markets: Vec<CoinsMarketItem> = CockoClient::coins_markets("jpy").unwrap();
        assert_eq!(!_coins_markets.is_empty(), true);
        assert_eq!(_coins_markets[0].id, "bitcoin");
    }

    #[test]
    fn call_coins_data() {
        let _coins_data: CoinsData = CockoClient::coins_with_id("bitcoin").unwrap();
        assert_eq!( _coins_data.name, "Bitcoin");
    }

    #[test]
    fn call_coins_data_ticker() {
        let _coins_ticker_data: CoinsTickerData = CockoClient::coins_with_id_tickers("bitcoin").unwrap();
        assert_eq!(_coins_ticker_data.name, "Bitcoin");
    }

    #[test]
    fn call_coins_data_history() {
        let _coins_history_data: CoinsHistoryData = CockoClient::coins_with_id_history("bitcoin", "30-12-2017").unwrap();
        assert_eq!(_coins_history_data.symbol, "btc");
    }

    #[test]
    fn call_coins_data_market_chart() {
        let _coins_market_chart: CoinsMarketChartData = CockoClient::coins_with_id_market_chart("bitcoin", "usd", "1").unwrap();
        assert_eq!(!_coins_market_chart.prices.is_empty(), true);
    }
    
}
