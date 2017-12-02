extern crate ini;

use self::ini::Ini;

const DEFAULT_SCHEME : &'static str = "http";
const DEFAULT_HOST : &'static str = "localhost";
const DEFAULT_PORT : &'static str = "5000";
const DEFAULT_PATH : &'static str = "/k2/public/api/1/carddata";

pub fn generate_url_from_config() -> String {
    let cfg;
    let mut scheme = DEFAULT_SCHEME;
    let mut host = DEFAULT_HOST;
    let mut port = DEFAULT_PORT;
    let mut path = DEFAULT_PATH;
    if let Ok(conf) = Ini::load_from_file("config.ini") {
        cfg = conf;
        if let Some(section) = cfg.section(Some("settings".to_owned())) {
            if let Some(config_val) = section.get("scheme") { scheme = config_val }
            if let Some(config_val) = section.get("host") { host = config_val }
            if let Some(config_val) = section.get("port") { port = config_val }
            if let Some(config_val) = section.get("path") { path = config_val }
        }
    }

    let url = format!("{}://{}:{}{}", scheme, host, port, path);
    url.to_owned()
}
