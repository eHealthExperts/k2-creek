extern crate base64;
extern crate der_parser;
extern crate encoding;
extern crate ini;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate reqwest;
#[macro_use]
extern crate rusticata_macros;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate treexml;

mod config;
mod creek_files;
mod file_writer;
mod kvk_data;
mod request;

lazy_static! {
    pub static ref CONFIG: config::Config = config::Config::new();
}

fn main() {
    let url = CONFIG.get_url();
    println!("Retrieving data from {}", &url);

    let res = request::fetch_egk_data(&url);

    file_writer::dump_egk_data_to_files(&res);
}
