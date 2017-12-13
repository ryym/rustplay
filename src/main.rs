extern crate rustplay;

fn main() {
    use std::io::Read;
    use std::fs::File;
    use std::env;
    use rustplay as rp;

    let args: Vec<String> = env::args().collect();
    let mut src = File::open(&args[1]).unwrap();

    let mut code = String::new();
    src.read_to_string(&mut code).unwrap();

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
