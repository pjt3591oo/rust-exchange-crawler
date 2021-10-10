use clap::{App, Arg};

#[derive(Debug)]
pub struct Params {
  symbol: String,
  count: i32,
  exchange: String,
}

pub fn parse() -> Result<Params, ()> {
  let matches = App::new("Exchange Crawler")
    .version("1.0")
    .author("JeongTae <pjt3591oo@gmail.com>")
    .about("crawler execution options")
    .arg(Arg::new("symbol")
      .short('s')
      .long("symbol")
      .value_name("SYMBOL")
      .about("Sets a crypto currency symbol")
      .required(true))
    .arg(Arg::new("count")
      .short('c')
      .long("count")
      .value_name("COUNT")
      .about("trading count")
      .required(true))
    .arg(Arg::new("exchange")
      .short('e')
      .long("exchange")
      .value_name("EXCHANGE")
      .about("upbit, bithumb, cobit")
      .required(true))
    .get_matches();

  let mut symbol: String = String::from("BTC");
  let mut count:i32 = 10;
  let mut exchange: String = String::from("upbit");

  if let Some(s) = matches.value_of("symbol") {
    symbol = s.to_string();
  }
  if let Some(c) = matches.value_of("count") {
    count = c.parse::<i32>().unwrap();
  }
  if let Some(e) = matches.value_of("exchange") {
    exchange = e.to_string();
  }

  Ok(Params {
    symbol,
    count,
    exchange,
  })
}