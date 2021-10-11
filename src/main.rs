mod cli;
mod exchange;

use crate::cli::{parse, Params};
use crate::exchange::{upbit, coinone, korbit, CrawlerInterface};

use upbit::{ Crawler as UpbitCrawler};
use coinone::{ Crawler as CoinoneCrawler };
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
  } else if params.exchange == "coinone" {
    let raw_text: String = CoinoneCrawler::new(params.count, &params.symbol).unwrap();
    let stocks: CoinoneCrawler = CoinoneCrawler::parse(&raw_text).unwrap();
    stocks.show();
  } else {
    let raw_text: String = KorbitCrawler::new(params.count, &params.symbol).unwrap();
    let stocks: KorbitCrawler = KorbitCrawler::parse(&raw_text).unwrap();
    stocks.show();
  }
}
