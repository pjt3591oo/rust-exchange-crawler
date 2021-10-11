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
$ ./exchange-crawler --help

Exchange Crawler 1.0

JeongTae <pjt3591oo@gmail.com>

crawler execution options

USAGE:
    exchange-crawler --symbol <SYMBOL> --count <COUNT> --exchange <EXCHANGE>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -c, --count <COUNT>          trading count
    -e, --exchange <EXCHANGE>    upbit, coinone, cobit
    -s, --symbol <SYMBOL>        Sets a crypto currency symbol
```

```bash
$ ./exchange-crawler --symbol BTC --count 2 --exchange coinone
```

upbit는 count로 결과 데이터 갯수 조정 가능하지만 coinone, korbit은 날짜 단위로 조회한다.


### Todo

count를 날짜 단위로 조회할 수 있도록 한다.