extern crate rustplay;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let ret = rustplay::cli::run(&args);
    match ret {
        Ok(Some(msg)) => println!("{}", msg),
        Ok(None) => {},
        Err(err) => {
            println!("{}", err.description());
            process::exit(1);
        },
    }
}
