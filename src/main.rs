extern crate error_chain;
extern crate rustplay;

use error_chain::ChainedError;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let ret = rustplay::cli::run(&args);
    match ret {
        Ok(Some(mut msg)) => print!("{}", ensure_newline(&mut msg)),
        Ok(None) => {}
        Err(err) => {
            let mut msg = format!("{}", err.display_chain());
            print!("{}", ensure_newline(&mut msg));
            process::exit(1);
        }
    }
}

fn ensure_newline(s: &mut String) -> &mut String {
    if !s.ends_with('\n') {
        s.push_str("\n");
    }
    s
}
