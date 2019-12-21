use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CoinListItem {
  id: String,
  symbol: String,
  name: String
}
