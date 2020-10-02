#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct Configuration {
    pub k2: K2,
    pub output: Output,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct K2 {
    pub api: u8,
    pub timeout: Option<u64>,
    pub url: Option<url::Url>,
}

impl Default for K2 {
    fn default() -> Self {
        Self {
            api: 2,
            timeout: None,
            url: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Output {
    pub force_delete: bool,
    pub path: std::path::PathBuf,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            force_delete: false,
            path: std::path::PathBuf::from("."),
        }
    }
}

impl Configuration {
    pub fn init(args: clap::ArgMatches<'_>) -> anyhow::Result<Self> {
        let mut settings = config::Config::new();

        if !args.is_present("url") {
            let config_file = match args.value_of_os("config-file") {
                Some(value) => value,
                None => std::ffi::OsStr::new("config.ini"),
            };
            let config_file_path: std::path::PathBuf = config_file.into();
            if !config_file_path.exists() {
                bail!(format!(
                    "Config file \"{}\" not found",
                    config_file_path.to_string_lossy()
                ))
            }

            // merge config file
            let config_file_source: config::File<config::FileSourceFile> = config_file_path.into();
            let _ = settings.merge(config_file_source).map_err(|error| {
                anyhow::Error::new(error).context("Failed to process config file!")
            })?;
        } else {
            settings.set("k2.url", args.value_of("url"))?;
            settings.set("output.force_delete", args.is_present("force_delete"))?;

            for setting in &[("api", "k2"), ("timeout", "k2"), ("path", "output")] {
                if let Some(value) = args.value_of(setting.0) {
                    settings.set(&format!("{}.{}", setting.1, setting.0), value)?;
                }
            }
        }

        let config: Self = settings.try_into().map_err(|error| {
            anyhow::Error::new(error).context("Incomplete configuration found!")
        })?;

        // ensure URL is set
        if config.k2.url.is_none() {
            bail!("Missing URL for K2!");
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn defaults() {
        let c = Configuration::default();

        assert_eq!(c.k2.api, 2);
        assert_eq!(c.k2.timeout, None);
        assert_eq!(c.k2.url, None);
        assert_eq!(c.output.force_delete, false);
        assert_eq!(c.output.path, std::path::PathBuf::from("."));
    }

    #[test]
    fn default_configuration() {
        let r = Configuration::init(default_args());

        assert!(r.is_ok());

        let config = r.unwrap();
        assert_eq!(config.k2.api, 2);
        assert_eq!(config.k2.timeout, None);
        assert_eq!(config.output.force_delete, false);
        assert_eq!(config.output.path, std::path::PathBuf::from("."));
    }

    #[test]
    #[ignore]
    fn no_valid_url_parameter() {
        assert!(Configuration::init(with_args(vec!["--url", "http://foo.bar"])).is_ok());

        let result = Configuration::init(with_args(vec!["--url", "foo.bar"]));
        assert!(result.is_err());

        let err = result.err().unwrap();
        let config_err = err.downcast_ref::<config::ConfigError>().unwrap();
        assert_eq!(
            config_err.to_string(),
            "invalid value: string \"foo.bar\", expected relative URL without a base"
        );
    }

    test! {
        name: config_file_missing,
        temp_dir: true,
        steps: {
            let result = Configuration::init(with_args(vec!["-c", "config.ini"]));
            assert!(result.is_err());

            let err = result.err().unwrap();
            assert_eq!(err.to_string(), "Config file \"config.ini\" not found");
        }
    }

    test! {
        name: config_file_overrides,
        temp_dir: true,
        steps: {
            let path = std::path::Path::new("config.ini");
            let mut file = std::fs::File::create(path).unwrap();

            use std::io::Write;
            writeln!(file, "[k2]").unwrap();
            writeln!(file, "api=3").unwrap();
            writeln!(file, "timeout=666").unwrap();
            writeln!(file, "url=http://foo.bar").unwrap();
            writeln!(file, "[output]").unwrap();
            writeln!(file, "force_delete=true").unwrap();
            writeln!(file, "path=c:/Temp").unwrap();

            let r = Configuration::init(with_args(vec![]));

            assert!(r.is_ok());

            let config = r.unwrap();
            assert_eq!(config.k2.api, 3);
            assert_eq!(config.k2.timeout, Some(666));
            assert_eq!(config.k2.url.unwrap().as_str(), "http://foo.bar/");
            assert_eq!(config.output.force_delete, true);
            assert_eq!(config.output.path, std::path::PathBuf::from("c:/Temp"));

            drop(file);
        }
    }

    fn default_args() -> clap::ArgMatches<'static> {
        with_args(vec!["--url", "http://example.com"])
    }

    fn with_args(args: Vec<&str>) -> clap::ArgMatches<'static> {
        clap::App::new("test")
            .setting(clap::AppSettings::NoBinaryName)
            .args(&crate::cli::args())
            .get_matches_from(args)
    }
}
