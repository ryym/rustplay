extern crate reqwest;
extern crate serde_json;
extern crate rustplay;

use rustplay::*;

fn main() {
    use std::io::{self, Read};
    use reqwest as r;

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let p = Payload{
        channel: "stable".to_string(),
        // code: "fn main() { println!(\"{}\", 1); }".to_string(),
        code: buf,
        crate_type: "bin".to_string(),
        mode: "debug".to_string(),
        tests: false,
    };

    let body = serde_json::to_string(&p).unwrap();

    let client = r::Client::new();
    let mut res = client.post("https://play.rust-lang.org/execute")
        .header(r::header::ContentType::json())
        .body(body)
        .send().unwrap();

    let ret: RunResult = res.json().unwrap();
    if ret.success {
        println!("{}", ret.stdout);
    } else {
        println!("{}", ret.stderr);
        std::process::exit(1);
    }
}
