mod cli;
mod exchange;
use cli::{parse, Params};

use crate::exchange::{upbit, bithumb, cobit, CrawlerInterface};
use upbit::{ Crawler as UpbitCrawler};
use bithumb::{ Crawler as bithumbCrawler };
use cobit::{ Crawler as cobitCrawler };

fn main() {
  let params: Params = match parse() {
    Ok(params) => params,
    Err(e) => {
      println!("{:?}", e);
      panic!("not match execute options: exchange, symbol, count")
    }
  };
  println!("{:?}", params);

  if params.exchange == "upbit" {
    let raw_text: String = UpbitCrawler::new(params.count, &params.symbol).unwrap();
    let stocks: UpbitCrawler = UpbitCrawler::parse(&raw_text).unwrap();
    stocks.show();
  } else if params.exchange == "bithumb" {
    let raw_text: String = bithumbCrawler::new(params.count, &params.symbol).unwrap();
    let stocks: bithumbCrawler = bithumbCrawler::parse(&raw_text).unwrap();
    stocks.show();
  } else {
    let raw_text: String = cobitCrawler::new(params.count, &params.symbol).unwrap();
    let stocks: cobitCrawler = cobitCrawler::parse(&raw_text).unwrap();
    stocks.show();
  }
  
  
}
