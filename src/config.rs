use serde_ini;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    #[serde(default)]
    settings: Settings,
}

#[derive(Debug, Deserialize)]
struct Settings {
    #[serde(default = "default_force_delete")]
    force_delete: bool,
    #[serde(default = "default_host")]
    host: String,
    #[serde(default = "default_path")]
    path: String,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_scheme")]
    scheme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scheme: default_scheme(),
            host: default_host(),
            port: default_port(),
            path: default_path(),
            force_delete: default_force_delete(),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let mut content = String::new();
        if let Ok(mut file) = File::open("config.ini") {
            let _ = file.read_to_string(&mut content);
        }

        serde_ini::from_str::<Configuration>(&content).expect("Failed to create configuration!")
    }
}

impl Configuration {
    pub fn get_url(&self) -> String {
        format!(
            "{}://{}:{}{}",
            &self.settings.scheme, &self.settings.host, &self.settings.port, &self.settings.path
        )
    }

    pub fn is_force_delete(&self) -> bool {
        self.settings.force_delete
    }
}

fn default_force_delete() -> bool {
    false
}

fn default_host() -> String {
    "localhost".to_owned()
}

fn default_path() -> String {
    "/k2/public/api/1/carddata".to_owned()
}

fn default_port() -> u16 {
    8089
}

fn default_scheme() -> String {
    "http".to_owned()
}
