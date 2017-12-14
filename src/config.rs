use errors::*;
use cli::CmdOpts;

pub struct Config {
    channel: Channel,
}

impl Config {
    pub fn new(opts: CmdOpts) -> Result<Config> {
        let channel = parse_channel(opts.channel)?;

        Ok(Config{
            channel,
        })
    }
}

fn parse_channel(ch: Option<String>) -> Result<Channel> {
    match ch {
        Some(ch) => match ch.as_ref() {
            "stable" => Ok(Channel::Stable),
            "beta" => Ok(Channel::Beta),
            "nightly" => Ok(Channel::Nightly),
            _ => Err("Invalid channel".into()),
        },
        None => Ok(Channel::Stable),
    }
}

pub enum Channel {
    Stable,
    Beta,
    Nightly,
}
