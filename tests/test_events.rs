use cocko_rs::client::CockoClient;

use cocko_rs::events::{
  EventCountriesParam,
  EventResponseParam,
  EventTypesParam
};


#[cfg(test)]
mod test_simple {
    use super::*;

    #[test]
    fn call_events() {
        let _events: EventResponseParam = CockoClient::events().unwrap();
        assert_eq!(_events.data.len() > 0, true);
    }

    #[test]
    fn call_events_countries() {
        let _events_countries: EventCountriesParam = CockoClient::events_countries().unwrap();
        assert_eq!(_events_countries.data.len() > 0, true);
    }

    #[test]
    fn call_events_types() {
        let _events_types: EventTypesParam = CockoClient::events_types().unwrap();
        assert_eq!(_events_types.data.len() > 0, true);
    }
}