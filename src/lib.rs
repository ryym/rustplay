#[macro_use]
extern crate serde_derive;
extern crate open;
extern crate reqwest;
extern crate serde_json;
extern crate urlencoding;
#[macro_use]
extern crate error_chain;
extern crate getopts;

pub mod cli;
pub mod client;
mod config;
mod errors;

pub use errors::{Error, Result};
