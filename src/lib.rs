#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate open;
extern crate urlencoding;
#[macro_use]
extern crate error_chain;
extern crate getopts;

mod config;
mod errors;
pub mod client;
pub mod cli;

pub use errors::{Error, Result};

#[derive(Deserialize, Debug)]
pub struct RunResult {
    pub stderr: String,
    pub stdout: String,
    pub success: bool,
}
