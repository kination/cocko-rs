use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EventResponseParam {
    pub data: Vec<EventDataItem>,
    pub count: i32,
    pub page: i32
}

#[derive(Debug, Deserialize)]
pub struct EventDataItem {
    pub r#type: String,
    pub title: String,
    pub description: String,
    pub organizer: String,
    pub start_date: String,
    pub end_date: String,
    pub website: String,
    pub email: String,
    pub venue: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub screenshot: String
}
