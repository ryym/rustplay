use errors::*;
use cli::CmdOpts;

pub struct Config {
    channel: Channel,
}

impl Config {
    pub fn new(opts: &CmdOpts) -> Result<Config> {
        let channel = parse_channel(&opts.channel)?;

        Ok(Config{
            channel,
        })
    }

    pub fn channel(&self) -> String {
        return self.channel.to_string()
    }
}

fn parse_channel(ch: &Option<String>) -> Result<Channel> {
    match *ch {
        Some(ref ch) => Channel::from(ch).ok_or("Invalid channel".into()),
        None => Ok(Channel::Stable),
    }
}

pub enum Channel {
    Stable,
    Beta,
    Nightly,
}

impl Channel {
    pub fn from(s: &str) -> Option<Self> {
        match s {
            "stable" => Some(Channel::Stable),
            "beta" => Some(Channel::Beta),
            "nightly" => Some(Channel::Nightly),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Channel::Stable => "stable",
            Channel::Beta => "beta",
            Channel::Nightly => "nightly",
        }.to_string()
    }

    pub fn possible_strs() -> String {
        use self::Channel::*;
        [Stable, Beta, Nightly]
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("|")
    }
}
