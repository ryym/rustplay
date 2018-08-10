use getopts;
use reqwest;
use serde_json;
use std::io;

error_chain! {
    foreign_links {
        SerdeJson(serde_json::Error);
        Reqwest(reqwest::Error);
        Io(io::Error);
        Opts(getopts::Fail);
    }
}
