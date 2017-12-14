use std::io::Read;
use std::fs::File;
use errors::*;
use config::Config;
use client;

pub struct CmdOpts {
    pub channel: Option<String>,
}

struct SrcFile(String);

pub fn run(args: Vec<String>) -> Result<String> {
    let (src, conf) = parse_args(args)?;
    exec(src, conf)
}

fn exec(src: SrcFile, _conf: Config) -> Result<String> {
    let SrcFile(filename) = src;
    let mut src = File::open(&filename)
        .chain_err(|| format!("Failed to open {}", &filename))?;

    let mut code = String::new();
    src.read_to_string(&mut code)
        .chain_err(|| format!("Failed to read {}", &filename))?;

    let client = client::new();
    let res = client.run(&code)?;

    if res.success {
        client.open(&code)?;
        Ok(res.stdout)
    } else {
        Err(res.stderr.into())
    }
}

fn parse_args(args: Vec<String>) -> Result<(SrcFile, Config)> {
    if args.len() <= 1 {
        return Err("No file name given".into())
    }

    let opts = CmdOpts {
        channel: Some("stable".to_string()),
    };

    Config::new(opts).map(|c| (SrcFile(args[1].clone()), c))
}

