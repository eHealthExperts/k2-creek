#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate rusticata_macros;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod file_writer;
#[macro_use]
mod files;
mod carddata;
mod config;
mod k2;

use crate::carddata::write_carddata;
use crate::config::Configuration;

lazy_static! {
    pub static ref CONFIG: Configuration = Configuration::default();
}

pub fn fetch_card_data() -> k2::Response {
    k2::request()
}

pub fn write_card_data(data: &k2::Response) {
    write_carddata(data)
}
