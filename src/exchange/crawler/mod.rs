use serde::{Serialize, Deserialize};

pub trait CrawlerInterface<T> {
  fn new(count: i32, symbol: &String) -> Result<String, reqwest::Error>;
  fn parse(raw_text: &str) -> Result<T, serde_json::Error>;
  fn show(&self);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockPacket {
  pub opening_price:  f64,
  pub trade_price:  f64,
  pub high_price:  f64,
  pub low_price:  f64,
}