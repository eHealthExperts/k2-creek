use config::{Config, File};
use failure::Error;

const CFG_FILE: &str = "config.ini";

#[derive(Debug, Deserialize, PartialEq)]
pub struct Configuration {
    pub settings: Settings,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Settings {
    pub force_delete: bool,
    pub host: String,
    pub path: String,
    pub port: u16,
    pub scheme: String,
}

impl Configuration {
    pub fn init() -> Result<Self, Error> {
        let mut settings = Config::new();

        // set defaults
        let _ = settings
            .set_default("force_delete", false)
            .expect("Failed to set default for force_delete!")
            .set_default("host", "localhost")
            .expect("Failed to set default for host!")
            .set_default("path", "/k2/public/api/1/carddata")
            .expect("Failed to set default for path!")
            .set_default("port", 8089)
            .expect("Failed to set default for port!")
            .set_default("scheme", "http")
            .expect("Failed to set default for scheme!");

        let _ = settings
            .merge(File::with_name(CFG_FILE).required(false))
            .expect("Failed to merge config file!");

        settings.try_into().map_err(failure::Error::from)
    }
}
