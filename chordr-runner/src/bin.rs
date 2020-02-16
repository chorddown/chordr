extern crate clap;
extern crate libchordr;
extern crate libsynchord;
extern crate log;

mod error;
mod configuration;

use crate::error::{Error, Result};
use clap::{App, Arg, ArgMatches};
use libchordr::prelude::{CatalogBuilder, FileType};
use libsynchord::error::Error as SynchordError;
use libsynchord::prelude::*;
use simplelog;
use simplelog::TerminalMode;
use std::env;
use std::fs;
use std::path::Path;
use configuration::reader::Reader;
use crate::configuration::Configuration;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daniel Corn <info@cundd.net>")
        .about("Synchronize chorddown files with online file services")
        .arg(Arg::with_name("configuration")
            .help("File containing the configuration")
            .short("c")
            .long("configuration")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("pretty")
            .help("Output indented JSON")
            .short("p")
            .long("pretty")
            .takes_value(true));

    let matches = app.get_matches();


    configure_logging(&matches)?;
    let configuration = read_configuration(&matches)?;
    download_and_build_catalog(&configuration, &matches)?;

    Ok(())
}

fn read_configuration(args: &ArgMatches) -> Result<Configuration> {
    Reader::read_configuration_from_file(&Path::new(args.value_of("configuration").unwrap()))
}

fn download_and_build_catalog(configuration: &Configuration, args: &ArgMatches) -> Result<()> {
    download(&configuration)?;
    build_catalog(&configuration, args)?;

    Ok(())
}

fn download(configuration: &Configuration) -> Result<()> {
    let service_config = build_service_config(configuration);
    let service = get_service(configuration, &service_config)?;

    libsynchord::helper::download(service, service_config)?;

    Ok(())
}

fn build_catalog(configuration: &Configuration, args: &ArgMatches) -> Result<()> {
    let pretty = args.is_present("pretty");

    let catalog = CatalogBuilder::new().build_catalog_for_directory(
        configuration.output_directory.as_path(),
        FileType::Chorddown,
        true,
    )?;

    let serialization_result = if pretty {
        serde_json::to_string_pretty(&catalog)
    } else {
        serde_json::to_string(&catalog)
    };

    let output = match serialization_result {
        Ok(s) => s,
        Err(e) => return Err(Error::serialization_error(format!("{}", e))),
    };

    Ok(fs::write(configuration.catalog_file.as_path(), output)?)
}

fn get_api_key() -> Result<String, SynchordError> {
    match env::var("API_TOKEN") {
        Ok(val) => Ok(val),
        Err(_) => Err(SynchordError::missing_argument_error(
            "No API token provided",
        )),
    }
}

fn get_password() -> Result<String, SynchordError> {
    match env::var("PASSWORD") {
        Ok(val) => Ok(val),
        Err(_) => Err(SynchordError::missing_argument_error(
            "No password provided",
        )),
    }
}

fn get_service(configuration: &Configuration, service_config: &ServiceConfig) -> Result<Services> {
    Ok(Services::build_service_by_identifier(
        &configuration.service.identifier.to_string(),
        service_config,
    )?)
}

fn build_service_config(configuration: &Configuration) -> ServiceConfig {
    let api_token = if !(configuration.service.api_token.trim().is_empty()) {
        Ok(configuration.service.api_token.trim().to_owned())
    } else {
        get_api_key()
    };

    let password = if !(configuration.service.password.trim().is_empty()) {
        Ok(configuration.service.password.trim().to_owned())
    } else {
        get_password()
    };
    ServiceConfig::new(
        api_token,
        Ok(configuration.service.url.clone()),
        Ok(configuration.service.remote_directory.clone()),
        Ok(configuration.service.username.clone()),
        password,
        Ok(configuration.output_directory.clone()),
    )
}

fn configure_logging(matches: &ArgMatches<'_>) -> Result<()> {
    let log_level_filter = match matches.occurrences_of("v") {
        1 => simplelog::LevelFilter::Info,
        2 => simplelog::LevelFilter::Debug,
        3 => simplelog::LevelFilter::Trace,
        _ => simplelog::LevelFilter::Warn,
    };

    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![];
    let mut config = simplelog::Config::default();
    config.time_format = Some("%H:%M:%S%.3f");

    if let Some(core_logger) =
    simplelog::TermLogger::new(log_level_filter, config, TerminalMode::Mixed)
    {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, config));
    }

    match simplelog::CombinedLogger::init(loggers) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::io_error(format!("{}", e))),
    }
}
