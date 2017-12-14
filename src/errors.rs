use std::io;
use serde_json;
use reqwest;

error_chain! {
    foreign_links {
        SerdeJson(serde_json::Error);
        Reqwest(reqwest::Error);
        Io(io::Error);
    }
}
