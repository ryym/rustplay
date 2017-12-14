extern crate rustplay;

use std::{env, process};

fn main() {
    let args = env::args().collect();
    let ret = rustplay::cli::run(args);
    match ret {
        Ok(msg) => {
            println!("{}", msg);
        },
        Err(err) => {
            println!("{}", err.description());
            process::exit(1);
        }
    }
}
