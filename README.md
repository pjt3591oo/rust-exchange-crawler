# exchange crawler

### build & run

* build

```
$ cargo build
```

* run

```bash
$ cd target/debug/
```

```bash
$ ./exchange-crawler --helpExchange Crawler 1.0

JeongTae <pjt3591oo@gmail.com>

crawler execution options

USAGE:
    exchange-crawler --symbol <SYMBOL> --count <COUNT> --exchange <EXCHANGE>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -c, --count <COUNT>          trading count
    -e, --exchange <EXCHANGE>    upbit, bithumb, cobit
    -s, --symbol <SYMBOL>        Sets a crypto currency symbol
```

```bash
$ ./exchange-crawler --symbol BTC --count 2 --exchange bithumb
```

upbit는 count로 결과 데이터 갯수 조정 가능하지만 bithumb, korbit은 날짜 단위로 조회한다.


### Todo

count를 날짜 단위로 조회할 수 있도록 한다.