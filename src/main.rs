mod cli;
mod exchange;
use cli::{parse, Params};

use crate::exchange::{upbit, bithumb, korbit, CrawlerInterface};
use upbit::{ Crawler as UpbitCrawler};
use bithumb::{ Crawler as BithumbCrawler };
use korbit::{ Crawler as KorbitCrawler };

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
    let raw_text: String = BithumbCrawler::new(params.count, &params.symbol).unwrap();
    let stocks: BithumbCrawler = BithumbCrawler::parse(&raw_text).unwrap();
    stocks.show();
  } else {
    let raw_text: String = KorbitCrawler::new(params.count, &params.symbol).unwrap();
    let stocks: KorbitCrawler = KorbitCrawler::parse(&raw_text).unwrap();
    stocks.show();
  }
}
