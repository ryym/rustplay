use cli::CmdOpts;
use errors::*;
use std::string::ToString;

pub struct Config {
    channel: Channel,
    mode: Mode,
}

impl Config {
    pub fn new(opts: &CmdOpts) -> Result<Config> {
        Ok(Config {
            channel: parse_channel(&opts.channel)?,
            mode: parse_mode(&opts.mode)?,
        })
    }

    pub fn channel(&self) -> String {
        return self.channel.to_string();
    }

    pub fn mode(&self) -> String {
        return self.mode.to_string();
    }
}

fn parse_channel(ch: &Option<String>) -> Result<Channel> {
    match *ch {
        Some(ref ch) => Channel::from(ch).ok_or("Invalid channel".into()),
        None => Ok(Channel::Stable),
    }
}

fn parse_mode(m: &Option<String>) -> Result<Mode> {
    match *m {
        Some(ref m) => Mode::from(m).ok_or("Invalid mode".into()),
        None => Ok(Mode::Debug),
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

    pub fn possible_strs() -> String {
        use self::Channel::*;
        join_enum_strings(&[Stable, Beta, Nightly], "|")
    }
}

impl ToString for Channel {
    fn to_string(&self) -> String {
        match *self {
            Channel::Stable => "stable",
            Channel::Beta => "beta",
            Channel::Nightly => "nightly",
        }.to_string()
    }
}

pub enum Mode {
    Debug,
    Release,
}

impl Mode {
    pub fn from(s: &str) -> Option<Self> {
        match s {
            "debug" => Some(Mode::Debug),
            "release" => Some(Mode::Release),
            _ => None,
        }
    }

    pub fn possible_strs() -> String {
        join_enum_strings(&[Mode::Debug, Mode::Release], "|")
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match *self {
            Mode::Debug => "debug",
            Mode::Release => "release",
        }.to_string()
    }
}

fn join_enum_strings<E: ToString>(items: &[E], sep: &str) -> String {
    items
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
