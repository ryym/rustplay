use std::io::Read;
use std::fs::File;
use getopts::{Options, Matches};
use errors::*;
use config::Config;
use client;

pub struct CmdOpts {
    pub channel: Option<String>,
}

struct SrcFile(String);

enum Parsed {
    Help(String),
    Go(SrcFile, Config),
}

pub fn run(args: &Vec<String>) -> Result<String> {
    match parse_args(&args)? {
        Parsed::Help(msg) => Ok(msg),
        Parsed::Go(src, conf) => exec(src, conf)
    }
}

fn exec(src: SrcFile, conf: Config) -> Result<String> {
    let SrcFile(filename) = src;
    let mut src = File::open(&filename)
        .chain_err(|| format!("Failed to open {}", &filename))?;

    let mut code = String::new();
    src.read_to_string(&mut code)
        .chain_err(|| format!("Failed to read {}", &filename))?;

    let client = client::new(conf);
    let res = client.run(&code)?;

    if res.success {
        client.open(&code)?;
        Ok(res.stdout)
    } else {
        Err(res.stderr.into())
    }
}

fn parse_args(args: &Vec<String>) -> Result<Parsed> {
    let program = &args[0];
    let args = &args[1..];

    let mut opts = Options::new();
    let m = define_opts(&mut opts).parse(args)?;

    if m.opt_present("h") {
        return Ok(Parsed::Help(make_usage(program, &opts)));
    }

    if !has_enough_args(&m) {
        return Err(make_usage(program, &opts).into());
    }

    let opts = CmdOpts {
        channel: Some("stable".to_string()),
    };

    let src_file = m.free[0].clone();
    Config::new(opts).map(|c| Parsed::Go(SrcFile(src_file), c))
}

fn define_opts(opts: &mut Options) -> &mut Options {
    opts.optflag("h", "help", "print this help message");
    opts.optflag("r", "run", "run given code using Rust Playground");
    opts.optflag("o", "open", "open Rust Playground with given code");
    opts
}

fn has_enough_args(m: &Matches) -> bool {
    !m.free.is_empty() && (m.opt_present("r") || m.opt_present("o"))
}

fn make_usage(program: &str, opts: &Options) -> String {
    let msg = format!(
        "Usage: {} [options] FILE\nNOTE: -r or -o is required",
        program
    );
    opts.usage(&msg)
}
