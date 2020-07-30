#![deny(unused_features)]
#![deny(deprecated)]
#![warn(dead_code)]
#![warn(rust_2018_idioms)]
#![warn(unused_variables)]
#![warn(unused_imports)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log_derive;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod file_writer;
#[macro_use]
mod files;
mod carddata;
mod config;
mod k2;

pub use crate::carddata::write_carddata;
use crate::config::Configuration;
use antidote::RwLock;

lazy_static! {
    pub(crate) static ref CONFIG: RwLock<Configuration> =
        RwLock::new(Configuration::init().expect("Failed to init configuration!"));
}

pub fn fetch_carddata() -> k2::Response {
    k2::request()
}
