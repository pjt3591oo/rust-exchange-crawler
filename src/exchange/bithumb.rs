use reqwest;
use serde::{Serialize, Deserialize};

pub use super::{CrawlerInterface, StockPacket};

#[derive(Serialize, Deserialize, Debug)]
struct OriginalPacketRoot {
  data: OriginalPacket,
}
#[derive(Serialize, Deserialize, Debug)]
struct OriginalPacket {
  o: Vec<String>,
  h: Vec<String>,
  l: Vec<String>,
  c: Vec<String>,
  v: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crawler {
  pub stocks: Vec<StockPacket>,
}

impl CrawlerInterface<Crawler> for Crawler {
  fn new(count: i32, symbol: &String) -> Result<String, reqwest::Error> {
    let url = format!("https://pub2.bithumb.com/public/candlesticknew_trview/C0565_C0100/30M",);
    let client = reqwest::blocking::Client::builder()
      .danger_accept_invalid_certs(true)
      .build().unwrap();

    let raw_text: String = client.get(url).send().unwrap().text().unwrap();

    Ok(raw_text)
  }

  fn parse(raw_text: &str) -> Result<Crawler, serde_json::Error> {
    let origin_data: OriginalPacketRoot = serde_json::from_str(&raw_text).unwrap();
    let mut packets: Vec<StockPacket> = Vec::new();
    
    for (i, x) in origin_data.data.v.iter().enumerate() {
      packets.push(StockPacket {
        opening_price: origin_data.data.o[i].parse::<f64>().unwrap(),
        trade_price: origin_data.data.c[i].parse::<f64>().unwrap(),
        high_price: origin_data.data.h[i].parse::<f64>().unwrap(),
        low_price: origin_data.data.l[i].parse::<f64>().unwrap(),
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
