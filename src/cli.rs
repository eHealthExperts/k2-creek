use clap::{App, Arg, ArgMatches, Result};

pub(crate) fn args<'a, 'b>() -> Vec<Arg<'a, 'b>> {
    vec![
        Arg::with_name("api")
            .long("api")
            .value_name("VERSION")
            .help("API Version of K2 to use [default: 2]")
            .takes_value(true)
            .possible_values(&["1", "2", "3"])
            .hide_possible_values(true)
            .requires("url"),
        Arg::with_name("config-file")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Path to config file [default: config.ini]")
            .takes_value(true),
        Arg::with_name("force_delete")
            .short("f")
            .help("Do not ask before deleting old files")
            .requires("url"),
        Arg::with_name("path")
            .short("o")
            .long("output")
            .value_name("DIRECTORY")
            .help("Output folder for generated files [default: .]")
            .takes_value(true)
            .requires("url"),
        Arg::with_name("timeout")
            .short("t")
            .long("timeout")
            .value_name("SECONDS")
            .help("Timeout for request [default: none]")
            .takes_value(true)
            .requires("url"),
        Arg::with_name("url")
            .long("url")
            .value_name("URL")
            .help("URL of K2 instance")
            .takes_value(true)
            .conflicts_with("config"),
    ]
}

pub fn matches() -> Result<ArgMatches<'static>> {
    App::new("K2-Creek")
        .version(crate_version!())
        .author("Made with ♥︎ by eHealthExperts GmbH")
        .args(&args())
        .get_matches_safe()
}

#[test]
fn required_argument_url() {
    for (arg, val) in [
        ("--api", Some("1")),
        ("-f", None),
        ("-o", Some("E:/TMP")),
        ("--output", Some("C:/Windows/temp")),
        ("-t", Some("1000")),
        ("--timeout", Some("1")),
    ]
    .iter()
    .cloned()
    .collect::<std::collections::HashMap<&str, Option<&str>>>()
    {
        let app = clap::App::new("test")
            .setting(clap::AppSettings::NoBinaryName)
            .args(&args());

        let result = match val {
            Some(val) => app.get_matches_from_safe(vec![arg, val]),
            None => app.get_matches_from_safe(vec![arg]),
        };

        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(err.kind, clap::ErrorKind::MissingRequiredArgument);
        debug!("Msg {}", err.message);
        assert!(err.message.contains("--url <URL>"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_values_for_api() {
        for (arg, valid) in [
            ("1", true),
            ("2", true),
            ("3", true),
            ("0", false),
            ("4", false),
        ]
        .iter()
        .cloned()
        .collect::<std::collections::HashMap<&str, bool>>()
        {
            let result = clap::App::new("test")
                .setting(clap::AppSettings::NoBinaryName)
                .args(&args())
                .get_matches_from_safe(vec!["--url", "http://foo.bar", "--api", arg]);

            if valid {
                assert!(result.is_ok());
            } else {
                assert!(result.is_err());

                let err = result.err().unwrap();
                assert_eq!(err.kind, clap::ErrorKind::InvalidValue);
                assert!(err.message.contains("isn't a valid value for"));
            }
        }
    }
}
