#[macro_use] extern crate serde_derive;

mod config;
mod file_writer;
mod request;

use config::generate_url_from_config;
use file_writer::dump_egk_data_to_files;
use request::request_egk_data;

fn main() {
    let url = generate_url_from_config();
    println!("Retrieving data from {}", &url);

    let resp = request_egk_data(&url).unwrap();

    dump_egk_data_to_files(&resp);
}
