use reqwest;
use serde::{Serialize, Deserialize};

pub use super::{CrawlerInterface, StockPacket};

#[derive(Serialize, Deserialize, Debug)]
struct OriginalPacket {
  o: Vec<f64>,
  h: Vec<f64>,
  l: Vec<f64>,
  c: Vec<f64>,
  v: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crawler {
  pub stocks: Vec<StockPacket>,
}


impl CrawlerInterface<Crawler> for Crawler {
  fn new(count: i32, symbol: &String) -> Result<String, reqwest::Error> {
    let url = format!("https://d1cta0eou52ius.cloudfront.net/{}_KRW_1m_20210827.json", &symbol);
    let client = reqwest::blocking::Client::builder()
      .danger_accept_invalid_certs(true)
      .build().unwrap();

    let raw_text: String = client.get(url).send().unwrap().text().unwrap();

    Ok(raw_text)
  }

  fn parse(raw_text: &str) -> Result<Crawler, serde_json::Error> {
    let origin_data: OriginalPacket = serde_json::from_str(&raw_text).unwrap();

    let mut packets: Vec<StockPacket> = Vec::new();
    
    for (i, x) in origin_data.v.iter().enumerate() {
      packets.push(StockPacket {
        opening_price: origin_data.o[i],
        trade_price: origin_data.c[i],
        high_price: origin_data.h[i],
        low_price: origin_data.l[i],
      });
    };
    
    Ok(Crawler {
      stocks: packets,
    })
  }

  fn show(&self) {
    for stock in &self.stocks {
      println!("tradePrice: {:?}, hightPrice: {:?}, lowPrice: {:?}, openningPrice: {:?}", stock.trade_price,  stock.high_price,  stock.low_price,  stock.opening_price);
    }
  }
}
