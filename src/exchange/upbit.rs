use reqwest;
use serde::{Serialize, Deserialize};

pub use super::{CrawlerInterface, StockPacket};

#[derive(Serialize, Deserialize, Debug)]
struct OriginalPacket {
  openingPrice:  f64,
  tradePrice:  f64,
  highPrice:  f64,
  lowPrice:  f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crawler {
  pub stocks: Vec<StockPacket>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeOrderRoot {
  data:TradeOrder,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeOrder {
  crncCd: String,
}

impl CrawlerInterface<Crawler> for Crawler {
  fn new(count: i32, symbol: &String) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
    .danger_accept_invalid_certs(true)
    .build().unwrap();

    let url = format!("https://crix-api-cdn.upbit.com/v1/crix/candles/minutes/30?code=CRIX.UPBIT.KRW-{}&count={}&ciqrandom=1633651016830", &symbol, count, );
    let raw_text: String = client.get(url).send().unwrap().text().unwrap();

    Ok(raw_text)
  }

  fn parse(raw_text: &str) -> Result<Crawler, serde_json::Error> {
    let origin_data: Vec<OriginalPacket> = serde_json::from_str(&raw_text).unwrap();

    let mut packets: Vec<StockPacket> = Vec::new();

    for (i, x) in origin_data.iter().enumerate() {
      packets.push(StockPacket {
        opening_price: x.openingPrice,
        trade_price: x.tradePrice,
        high_price: x.highPrice,
        low_price: x.lowPrice,
      });
    };

    Ok(Crawler{stocks: packets})
  }

  fn show(&self) {
      for stock in &self.stocks {
        println!("tradePrice: {:?}, hightPrice: {:?}, lowPrice: {:?}, openningPrice: {:?}", stock.trade_price,  stock.high_price,  stock.low_price,  stock.opening_price);
    }
  }
}
