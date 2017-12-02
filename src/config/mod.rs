extern crate ini;

use self::ini::Ini;

const DEFAULT_SCHEME : &'static str = "http";
const DEFAULT_HOST : &'static str = "localhost";
const DEFAULT_PORT : &'static str = "5000";
const DEFAULT_PATH : &'static str = "/k2/public/api/1/carddata";

pub fn generate_url_from_config() -> String {
    let conf = Ini::load_from_file("config.ini").unwrap();
    let section = conf.section(Some("settings".to_owned())).unwrap();
    let def_scheme = DEFAULT_SCHEME.to_owned();
    let scheme = section.get("scheme").unwrap_or(&def_scheme);
    let def_host = DEFAULT_HOST.to_owned();
    let host = section.get("host").unwrap_or(&def_host);
    let def_port = DEFAULT_PORT.to_owned();
    let port = section.get("port").unwrap_or(&def_port);
    let def_path = DEFAULT_PATH.to_owned();
    let path = section.get("path").unwrap_or(&def_path);
    let url = format!("{}://{}:{}{}", scheme, host, port, path);
    url.to_owned()
}
