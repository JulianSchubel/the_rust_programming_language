use std::env;
use minigrep::prelude::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    println!("{}", minigrep(config.query, config.filename));
}

