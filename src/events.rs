use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EventResponseParam {
  pub data: Vec<EventDataItem>,
  count: i32,
  page: i32
}

#[derive(Debug, Deserialize)]
pub struct EventDataItem {
  r#type: String,
  title: String,
  description: String,
  organizer: String,
  start_date: String,
  end_date: String,
  website: String,
  email: String,
  venue: String,
  address: String,
  city: String,
  country: String,
  screenshot: String
}
