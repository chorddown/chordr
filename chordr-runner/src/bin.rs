extern crate clap;
extern crate libchordr;
extern crate libsynchord;
extern crate log;

mod error;

use crate::error::{Error, Result};
use clap::{App, Arg, ArgMatches, SubCommand};
use libchordr::prelude::{CatalogBuilder, FileType};
use libsynchord::error::Error as SynchordError;
use libsynchord::prelude::*;
use simplelog;
use simplelog::TerminalMode;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let catalog_file_output_arg = Arg::with_name("CATALOG_FILE")
        .required(true)
        .help("Catalog output file name");

    let output_arg = Arg::with_name("OUTPUT")
        .required(true)
        .help("Output directory path");
    let service_arg = Arg::with_name("SERVICE")
        .required(true)
        .help("Online service to use (dropbox, WebDAV)");
    let api_token_arg = Arg::with_name("API_TOKEN")
        .long("api-key")
        .takes_value(true)
        .help("API key to authenticate with the service");
    let username_arg = Arg::with_name("USERNAME")
        .long("username")
        .short("u")
        .takes_value(true)
        .help("Username to authenticate with the service");
    let password_arg = Arg::with_name("PASSWORD")
        .long("password")
        .short("p")
        .takes_value(true)
        .help("Password to authenticate with the service");
    let url_arg = Arg::with_name("URL")
        .long("url")
        .takes_value(true)
        .help("WebDAV entry point URL");
    let remote_directory_arg = Arg::with_name("REMOTE_DIRECTORY")
        .long("remote-directory")
        .takes_value(true)
        .help("Remote directory to list");
    let pretty_arg = Arg::with_name("pretty")
        .long("pretty")
        .help("Output indented JSON");
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daniel Corn <info@cundd.net>")
        .about("Synchronize chorddown files with online file services")
        .subcommand(
            SubCommand::with_name("download")
                .version(env!("CARGO_PKG_VERSION"))
                .about("Download files from the service")
                .arg(catalog_file_output_arg.clone())
                .arg(service_arg.clone())
                .arg(output_arg.clone())
                .arg(api_token_arg.clone())
                .arg(username_arg.clone())
                .arg(password_arg.clone())
                .arg(url_arg.clone())
                .arg(remote_directory_arg.clone())
                .arg(pretty_arg.clone()),
        )
        .get_matches();

    if let Err(error) = configure_logging(&args) {
        eprintln!("{}", error);
    }
    let error = if let Some(matches) = args.subcommand_matches("download") {
        download_and_build_catalog(matches)
    } else {
        eprintln!("Missing argument 'subcommand'");
        Ok(())
    };

    if let Err(error) = error {
        eprintln!("{:?}", error)
    }
}

fn download_and_build_catalog(args: &ArgMatches) -> Result<()> {
    download(args)?;
    build_catalog(args)?;

    Ok(())
}

fn download(args: &ArgMatches) -> Result<()> {
    let service_config = build_service_config(args);
    let service = get_service(args, &service_config)?;

    libsynchord::helper::download(service, service_config)?;

    Ok(())
}

fn build_catalog(args: &ArgMatches) -> Result<()> {
    let pretty = args.is_present("pretty");

    let catalog = CatalogBuilder::new().build_catalog_for_directory(
        get_local_directory(args)?,
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

    handle_output(args, output)
}

fn handle_output(args: &ArgMatches, output: String) -> Result<(), Error> {
    let output_file_path = args.value_of("CATALOG_FILE").unwrap();
    if output_file_path == "-" {
        println!("{}", output);
        Ok(())
    } else {
        Ok(fs::write(output_file_path, output)?)
    }
}

fn get_api_key(args: &ArgMatches) -> Result<String, SynchordError> {
    if let Some(t) = args.value_of("API_TOKEN") {
        return Ok(t.to_owned());
    }

    match env::var("API_TOKEN") {
        Ok(val) => Ok(val),
        Err(_) => Err(SynchordError::missing_argument_error(
            "No API token provided",
        )),
    }
}

fn get_url(args: &ArgMatches) -> Result<String, SynchordError> {
    match args.value_of("URL") {
        Some(val) => Ok(val.to_owned()),
        None => Err(SynchordError::missing_argument_error("No URL provided")),
    }
}

fn get_username(args: &ArgMatches) -> Result<String, SynchordError> {
    match args.value_of("USERNAME") {
        Some(val) => Ok(val.to_owned()),
        None => Err(SynchordError::missing_argument_error(
            "No username provided",
        )),
    }
}

fn get_remote_directory(args: &ArgMatches) -> Result<String, SynchordError> {
    match args.value_of("REMOTE_DIRECTORY") {
        Some(val) => Ok(val.to_owned()),
        None => Err(SynchordError::missing_argument_error(
            "No remote directory provided",
        )),
    }
}

fn get_password(args: &ArgMatches) -> Result<String, SynchordError> {
    if let Some(t) = args.value_of("PASSWORD") {
        return Ok(t.to_owned());
    }

    match env::var("PASSWORD") {
        Ok(val) => Ok(val),
        Err(_) => Err(SynchordError::missing_argument_error(
            "No password provided",
        )),
    }
}

fn get_service(args: &ArgMatches, service_config: &ServiceConfig) -> Result<Services> {
    let service_identifier = args.value_of("SERVICE").unwrap();

    Ok(Services::build_service_by_identifier(
        service_identifier,
        service_config,
    )?)
}

fn get_local_directory(args: &ArgMatches) -> Result<PathBuf, SynchordError> {
    let output_path = PathBuf::from(args.value_of("OUTPUT").unwrap());
    let output_path_string = output_path.to_str().map_or_else(
        || format!("{}", output_path.to_string_lossy()),
        |s| s.to_owned(),
    );

    if output_path.is_dir() {
        Ok(output_path)
    } else if output_path.is_file() {
        Err(SynchordError::io_error(format!(
            "Output path {} is not a directory",
            output_path_string
        )))
    } else if !output_path.exists() {
        Err(SynchordError::io_error(format!(
            "Output path {} does not exist",
            output_path_string
        )))
    } else {
        Err(SynchordError::io_error(format!(
            "Output path {} is not a path",
            output_path_string
        )))
    }
}

fn build_service_config(args: &ArgMatches) -> ServiceConfig {
    ServiceConfig::new(
        get_api_key(args),
        get_url(args),
        get_remote_directory(args),
        get_username(args),
        get_password(args),
        get_local_directory(args),
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
