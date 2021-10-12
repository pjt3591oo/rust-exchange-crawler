use reqwest;
use serde::{Serialize, Deserialize};

pub use super::{CrawlerInterface, StockPacket};

#[derive(Serialize, Deserialize, Debug)]
struct OriginalPacketRoot {
  data: Vec<OriginalPacket>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalPacket {
  Close: String,
  High: String,
  Low: String,
  Open: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crawler {
  pub stocks: Vec<StockPacket>,
}

impl CrawlerInterface<Crawler> for Crawler {
  fn new(_count: i32, _symbol: &String) -> Result<String, reqwest::Error> {
    let symbol: &str = if _symbol == "BTC" {
      ""
    } else {
      _symbol
    };
    
    let url: String = format!("https://tb.coinone.co.kr/api/v1/chart/olhc/?site=coinone{}&type=1m&last_time=1633903440000", &symbol);

    let client = reqwest::blocking::Client::builder()
      .danger_accept_invalid_certs(true)
      .build().unwrap();

    let raw_text: String = client.get(&url).send().unwrap().text().unwrap();
    Ok(raw_text)
  }

  fn parse(raw_text: &str) -> Result<Crawler, serde_json::Error> {
    let origin_data: OriginalPacketRoot = serde_json::from_str(&raw_text).unwrap();
    let mut packets: Vec<StockPacket> = Vec::new();
    
    for (i, x) in origin_data.data.iter().enumerate() {
      packets.push(StockPacket {
        opening_price: x.Open.parse::<f64>().unwrap(),
        trade_price: x.Close.parse::<f64>().unwrap(),
        high_price: x.High.parse::<f64>().unwrap(),
        low_price: x.Low.parse::<f64>().unwrap(),
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
