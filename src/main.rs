extern crate rustplay;

fn main() {
    use std::io::{self, Read};
    use rustplay as rp;

    let mut code = String::new();
    io::stdin().read_to_string(&mut code).unwrap();
    let code = code;
    let client = rp::client::new();
    let res = client.run(&code);

    if res.success {
        println!("{}", res.stdout);
        client.open(&code);
    } else {
        println!("{}", res.stderr);
        std::process::exit(1);
    }
}
