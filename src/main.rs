mod cli;
use cli::{parse};

fn main() {
    let args = match parse() {
        Ok(args) => args,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };
    println!("{:?}", args);


}
