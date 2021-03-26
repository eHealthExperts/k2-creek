#![deny(unused_features)]
#![deny(deprecated)]
#![warn(dead_code)]
#![warn(unused_variables)]
#![warn(unused_imports)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate serial_test;

#[cfg(test)]
#[macro_use]
mod tests;

mod api;
mod cli;
mod config;
mod egk;
mod http;
mod kvk;
mod writer;

pub fn start() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env().init();

    let args = crate::cli::matches().unwrap_or_else(|error| error.exit());
    let config = config::Configuration::init(args)?;
    trace!("{:?}", config);

    std::env::set_current_dir(&config.output.path).map_err(|error| {
        anyhow::Error::new(error).context(format!(
            "Unable to open output path: {}",
            config.output.path.as_path().to_string_lossy()
        ))
    })?;

    use writer::WriteApi;
    let data: api::Api = http::get(config.k2.url.unwrap(), config.k2.api, config.k2.timeout)?;
    debug!("Deserialized {:?}", data);

    data.write(config.output.force_delete)
}
