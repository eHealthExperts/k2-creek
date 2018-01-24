macro_rules! replace_by_ini {
    ($ini:ident, $target:expr, $config_key:expr) => (
        if let Some(val) = $ini.get_from(Some("settings"), $config_key) {
            $target  = val.to_owned();
        }
    )
}

pub struct Config {
    scheme: String,
    host: String,
    port: String,
    path: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            scheme: String::from("http"),
            host: String::from("localhost"),
            port: String::from("5000"),
            path: String::from("/k2/public/api/1/carddata"),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        let mut config: Config = Default::default();

        if let Ok(ini) = ::ini::Ini::load_from_file("config.ini") {
            replace_by_ini!(ini, config.scheme, "scheme");
            replace_by_ini!(ini, config.host, "host");
            replace_by_ini!(ini, config.port, "port");
            replace_by_ini!(ini, config.path, "path");
        }

        config
    }

    pub fn to_url(&self) -> String {
        format!(
            "{}://{}:{}{}",
            &self.scheme, &self.host, &self.port, &self.path
        )
    }
}
