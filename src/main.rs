extern crate base64;
extern crate encoding;
extern crate ini;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate treexml;

mod config;
mod creek_files;
mod file_writer;
mod request;

lazy_static! {
    pub static ref CONFIG: config::Config = config::Config::new();
}

fn main() {
    let url = CONFIG.get_url();
    println!("Retrieving data from {}", &url);

    let res = request::request_egk_data(&url);
    println!("Processing response...");

    file_writer::dump_egk_data_to_files(&res);
}
