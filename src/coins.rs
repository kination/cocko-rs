use std::collections::HashMap;

use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CoinsListItem {
    pub id: String,
    pub symbol: String,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct CoinsMarketItem {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: f32,
    pub market_cap: i64,
    pub market_cap_rank: i32,
    pub total_volume: i64,
    pub high_24h: f32,
    pub low_24h: f32,
    pub price_change_24h: f64,
    pub price_change_percentage_24h: f64,
    pub market_cap_change_24h: f64,
    pub market_cap_change_percentage_24h: f64,
    // #[serde(deserialize_with = "deserialize_number_field")]
    pub circulating_supply: f64,
    pub total_supply: Option<f64>,
    pub ath: f64,
    pub ath_change_percentage: f64,
    pub ath_date: String,
    pub roi: Option<CoinsMarketRoi>,
    pub last_updated: String,
}

#[derive(Debug, Deserialize)]
pub struct CoinsMarketRoi {
    pub times: f64,
    pub currency: String,
    pub percentage: f64
}

#[derive(Debug, Deserialize)]
pub struct CoinsData {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub asset_platform_id: Option<String>,
    pub block_time_in_minutes: i32,
    pub hashing_algorithm: String,
    pub categories: Vec<String>,
    pub localization: HashMap<String, String>,
    pub description: HashMap<String, String>,
    pub links: LinkType,
    pub image: HashMap<String, String>,
    pub country_origin: String,
    pub genesis_date: String,
    pub sentiment_votes_up_percentage: f32,
    pub sentiment_votes_down_percentage: f32,
    pub market_cap_rank: i32,
    pub coingecko_rank: i32,
    pub coingecko_score: f64,
    pub developer_score: f64,
    pub community_score: f64,
    pub liquidity_score: f64,
    pub public_interest_score: f64,
    pub market_data: MarketDataType,
    pub community_data: CommunityDataType,
    pub developer_data: DeveloperDataType,
    pub public_interest_stats: HashMap<String, i64>,
    pub status_updates: Vec<String>,
    pub last_updated: String,
    pub tickers: Vec<TickersType>
}

#[derive(Debug, Deserialize)]
pub struct CoinsTickerData {
    pub name: String,
    pub tickers: Vec<TickersType>
}

#[derive(Debug, Deserialize)]
pub struct CoinsHistoryData {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub localization: HashMap<String, String>,
    pub image: HashMap<String, String>,
    pub market_data: HistoryMarketDataType,
    pub community_data: CommunityDataType,
    pub developer_data: DeveloperDataType,
    pub public_interest_stats: HashMap<String, i64>
}

#[derive(Debug, Deserialize)]
pub struct CoinsMarketChartData {
    pub prices: Vec<[f64; 2]>,
    pub market_caps: Vec<[f64; 2]>,
    pub total_volumes: Vec<[f64; 2]>
}

#[derive(Debug, Deserialize)]
pub struct LinkType {
    pub homepage: Vec<String>,
    pub blockchain_site: Vec<String>,
    pub official_forum_url: Vec<String>,
    pub chat_url: Vec<String>,
    pub announcement_url: Vec<String>,
    pub twitter_screen_name: String,
    pub facebook_username: String,
    pub bitcointalk_thread_identifier: Option<String>,
    pub telegram_channel_identifier: String,
    pub subreddit_url: String,
    pub repos_url: HashMap<String, Vec<String>>
}

#[derive(Debug, Deserialize)]
pub struct MarketDataType {
    pub current_price: HashMap<String, f32>,
    pub roi: Option<CoinsMarketRoi>,
    pub ath: HashMap<String, f32>,
    pub ath_change_percentage: HashMap<String, f32>,
    pub ath_date: HashMap<String, String>,
    pub atl: HashMap<String, f32>,
    pub atl_change_percentage: HashMap<String, f32>,
    pub atl_date: HashMap<String, String>,
    pub market_cap: HashMap<String, f32>,
    pub market_cap_rank: i32,
    pub total_volume: HashMap<String, i64>,
    pub high_24h: HashMap<String, f32>,
    pub low_24h: HashMap<String, f32>,
    pub price_change_24h: f64,
    pub price_change_percentage_24h: f64,
    pub price_change_percentage_7d: f64,
    pub price_change_percentage_14d: f64,
    pub price_change_percentage_30d: f64,
    pub price_change_percentage_60d: f64,
    pub price_change_percentage_200d: f64,
    pub price_change_percentage_1y: f64,
    pub market_cap_change_24h: f64,
    pub market_cap_change_percentage_24h: f64,
    pub price_change_24h_in_currency: HashMap<String, f64>,
    pub price_change_percentage_1h_in_currency: HashMap<String, f64>,
    pub price_change_percentage_7d_in_currency: HashMap<String, f64>,
    pub price_change_percentage_14d_in_currency: HashMap<String, f64>,
    pub price_change_percentage_30d_in_currency: HashMap<String, f64>,
    pub price_change_percentage_60d_in_currency: HashMap<String, f64>,
    pub price_change_percentage_200d_in_currency: HashMap<String, f64>,
    pub price_change_percentage_1y_in_currency: HashMap<String, f64>,
    pub market_cap_change_24h_in_currency: HashMap<String, f64>,
    pub market_cap_change_percentage_24h_in_currency: HashMap<String, f64>,
    pub total_supply: f64,
    pub circulating_supply: f64,
    pub last_updated: String,
}

#[derive(Debug, Deserialize)]
pub struct HistoryMarketDataType {
    pub current_price: HashMap<String, f64>,
    pub market_cap: HashMap<String, f64>,
    pub total_volume: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct CommunityDataType {
    pub facebook_likes: Option<i64>,
    pub twitter_followers: Option<i64>,
    pub reddit_average_posts_48h: Option<f64>,
    pub reddit_average_comments_48h: Option<f64>,
    pub reddit_subscribers: Option<i64>,
    pub reddit_accounts_active_48h: Option<String>,
    pub telegram_channel_user_count: Option<i64>
}

#[derive(Debug, Deserialize)]
pub struct DeveloperDataType {
    pub forks: i32,
    pub stars: i32,
    pub subscribers: i32,
    pub total_issues: i32,
    pub closed_issues: i32,
    pub pull_requests_merged: i32,
    pub pull_request_contributors: i32,
    pub code_additions_deletions_4_weeks: HashMap<String, Option<i32>>,
    pub commit_count_4_weeks: i32,
    pub last_4_weeks_commit_activity_series: Option<Vec<i32>>
}

#[derive(Debug, Deserialize)]
pub struct TickersType {
    pub base: String,
    pub target: String,
    pub market: TickersMarketType,
    pub last: f64,
    pub volume: f64,
    pub converted_last: HashMap<String, f64>,
    pub converted_volume: HashMap<String, f64>,
    pub trust_score: String,
    pub bid_ask_spread_percentage: f64,
    pub timestamp: String,
    pub last_traded_at: String,
    pub last_fetch_at: String,
    pub is_anomaly: bool,
    pub is_stale: bool,
    pub trade_url: Option<String>,
    pub coin_id: String,
    pub target_coin_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TickersMarketType {
    pub name: String,
    pub identifier: String,
    pub has_trading_incentive: bool
}

/*
fn deserialize_number_field<'de, D>(deserializer: D) -> Result<f64, D::Error>
        where D: Deserializer<'de>,
    {
        // TODO
    }
*/
