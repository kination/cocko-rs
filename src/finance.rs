use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FinancePlatformItem {
    name: String,
    facts: String,
    category: String,
    centralized: bool,
    website_url: String
}
