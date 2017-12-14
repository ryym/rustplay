use std::io::Read;
use std::fs::File;
use errors::*;
use client;

pub fn run(args: Vec<String>) -> Result<String> {
    if args.len() <= 1 {
        return Err("No file name given".into())
    }

    let filename = &args[1];
    let mut src = File::open(filename)
        .chain_err(|| format!("Failed to open {}", filename))?;

    let mut code = String::new();
    src.read_to_string(&mut code)
        .chain_err(|| format!("Failed to read {}", filename))?;

    let client = client::new();
    let res = client.run(&code)?;

    if res.success {
        client.open(&code)?;
        Ok(res.stdout)
    } else {
        Err(res.stderr.into())
    }
}

