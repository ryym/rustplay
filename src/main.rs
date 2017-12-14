extern crate rustplay;

fn main() {
    use std::io::Read;
    use std::fs::File;
    use std::env;
    use rustplay as rp;

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No file name given");
        std::process::exit(1);
    }

    let mut src = File::open(&args[1]).unwrap();

    let mut code = String::new();
    src.read_to_string(&mut code).unwrap();

    let client = rp::client::new();
    let res = client.run(&code)
        .expect("Failed to run code on the Playground");

    if res.success {
        println!("{}", res.stdout);
        client.open(&code)
            .expect("Failed to open Rust Playground");
    } else {
        println!("{}", res.stderr);
        std::process::exit(1);
    }
}
