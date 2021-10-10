pub trait CrawlerInterface<T> {
  fn new(count: i32, symbol: &String) -> Result<String, reqwest::Error>;
  fn parse(raw_text: &str) -> Result<T, serde_json::Error>;
  fn show(&self);
}