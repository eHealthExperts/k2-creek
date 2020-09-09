#[cfg(unix)]
const EXECUTABLE: &str = "k2-creek";
#[cfg(windows)]
const EXECUTABLE: &str = "k2-creek.exe";

#[test]
fn help_is_displayed() {
    use predicates::prelude::*;

    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--help")
        .assert()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::similar(format!(
            "K2-Creek {}
Made with ♥︎ by eHealthExperts GmbH

USAGE:
    {} [FLAGS] [OPTIONS]

FLAGS:
    -f               Do not ask before deleting old files
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --api <VERSION>         API Version of K2 to use [default: 2]
    -c, --config <FILE>         Path to config file [default: config.ini]
    -o, --output <DIRECTORY>    Output folder for generated files [default: .]
    -t, --timeout <SECONDS>     Timeout for request [default: none]
        --url <URL>             URL of K2 instance
",
            env!("CARGO_PKG_VERSION"),
            EXECUTABLE
        )))
        .success();
}

#[test]
fn usage_is_displayed() {
    for arg in &[
        ("--api", Some("2"), "--api <VERSION>"),
        ("-f", None, "-f"),
        ("-o", Some("./test"), "--output <DIRECTORY>"),
        ("--output", Some("/tmp"), "--output <DIRECTORY>"),
        ("-t", Some("6"), "--timeout <SECONDS>"),
        ("--timeout", Some("222"), "--timeout <SECONDS>"),
    ] {
        use predicates::prelude::*;
        let mut command = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        command.arg(arg.0);

        if let Some(value) = arg.1 {
            command.arg(value);
        }

        command
            .assert()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::similar(format!(
                "error: The following required arguments were not provided:
    --url <URL>

USAGE:
    {} {} --url <URL>

For more information try --help
",
                EXECUTABLE, arg.2
            )))
            .failure();
    }
}
