extern crate base64;
extern crate encoding;
extern crate ini;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate treexml;

mod config;
mod file_writer;
mod request;

fn main() {
    let config = config::Config::new();
    let url = config.to_url();
    println!("Retrieving data from {}", &url);

    let res = request::request_egk_data(&url);

    file_writer::dump_egk_data_to_files(&res);
}
