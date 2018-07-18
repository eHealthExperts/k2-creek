macro_rules! replace_by_ini {
    ($ini:ident, $target:expr, $config_key:expr) => {
        if let Some(val) = $ini.get_from(Some("settings"), $config_key) {
            $target = val.to_owned();
        }
    };
}

pub struct Config {
    scheme: String,
    host: String,
    port: String,
    path: String,
    force_delete: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            scheme: "http".to_owned(),
            host: "localhost".to_owned(),
            port: "8089".to_owned(),
            path: "/k2/public/api/1/carddata".to_owned(),
            force_delete: "false".to_owned(),
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
            replace_by_ini!(ini, config.force_delete, "force_delete");
        }
        match config.force_delete.parse::<bool>() {
            Err(_) => panic!("Invalid config value for force_delete. Must be true or false (which is the default),
                             but was {:?}", config.force_delete),
            _ => {}
        }
        config
    }

    pub fn get_url(&self) -> String {
        format!(
            "{}://{}:{}{}",
            &self.scheme, &self.host, &self.port, &self.path
        )
    }

    pub fn is_force_delete(&self) -> bool {
        self.force_delete.parse::<bool>().unwrap() // safe due to early validation in new()
    }
}
