use std::io::Read;
use std::fs::File;
use getopts::{Options, Matches};
use errors::*;
use config::{Config, Channel, Mode};
use client::Client;

pub struct CmdOpts {
    pub filename: String,
    pub run: bool,
    pub open: bool,
    pub channel: Option<String>,
    pub mode: Option<String>,
}

enum Parsed {
    Help(String),
    Go(CmdOpts, Config),
}

pub fn run(args: &Vec<String>) -> Result<Option<String>> {
    match parse_args(&args)? {
        Parsed::Help(msg) => Ok(Some(msg)),
        Parsed::Go(opts, conf) => exec(opts, conf).map(|_| None),
    }
}

fn exec(opts: CmdOpts, conf: Config) -> Result<()> {
    let filename = &opts.filename;
    let mut src = File::open(filename)
        .chain_err(|| format!("Failed to open {}", filename))?;

    let mut code = String::new();
    src.read_to_string(&mut code)
        .chain_err(|| format!("Failed to read {}", filename))?;

    let client = Client::new(conf);

    if opts.run {
        let res = client.run(&code)?;
        print!("{}", res.stdout);
    }
    if opts.open {
        client.open(&code)?;
    }
    Ok(())
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
        filename: m.free[0].clone(),
        run: m.opt_present("r"),
        open: m.opt_present("o"),
        channel: m.opt_str("channel"),
        mode: m.opt_str("mode"),
    };

    Config::new(&opts).map(|c| Parsed::Go(opts, c))
}

fn define_opts(opts: &mut Options) -> &mut Options {
    opts.optflag("h", "help", "print this help message");
    opts.optflag("r", "run", "compile and run given code using Rust Playground");
    opts.optflag("o", "open", "open Rust Playground with given code");
    opts.optopt("c", "channel",
                "chose release channel which compiles code",
                &Channel::possible_strs());
    opts.optopt("m", "mode",
                "chose compilation mode",
                &Mode::possible_strs());
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
