use config::{Config, File};
use failure::Error;

const CFG_FILE: &str = "config.ini";

#[derive(Debug, Deserialize, PartialEq)]
pub struct Configuration {
    pub k2: K2,
    pub output: Output,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct K2 {
    pub host: String,
    pub path: String,
    pub port: u16,
    pub scheme: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Output {
    pub force_delete: bool,
    pub path: String,
}

impl Configuration {
    #[logfn(
        ok = "TRACE",
        err = "Error",
        fmt = "Failed to create configuration: {:?}"
    )]
    pub fn init() -> Result<Self, Error> {
        let mut settings = Config::new();

        // set defaults
        let _ = settings
            .set_default("k2.host", "localhost")
            .expect("Failed to set default for host!")
            .set_default("k2.path", "/k2/public/api/1/carddata")
            .expect("Failed to set default for path!")
            .set_default("k2.port", 8089)
            .expect("Failed to set default for port!")
            .set_default("k2.scheme", "http")
            .expect("Failed to set default for scheme!")
            .set_default("output.force_delete", false)
            .expect("Failed to set default for force_delete!")
            .set_default("output.path", ".")
            .expect("Failed to set defautt path for output");

        let _ = settings
            .merge(File::with_name(CFG_FILE).required(false))
            .expect("Failed to merge config file!");

        settings.try_into().map_err(failure::Error::from)
    }
}
