extern crate clap;
extern crate log;

use crate::error::{Error, Result};
use crate::service::*;
use chrono::{DateTime, Utc};
use clap::{App, Arg, ArgMatches, SubCommand};
use log::{debug, info};
use simplelog;
use simplelog::TerminalMode;
use std::env;
use std::path::{Path, PathBuf};

mod error;
mod service;

fn main() {
    let output_arg = Arg::with_name("OUTPUT")
        .required(true)
        .help("Output directory path");
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daniel Corn <info@cundd.net>")
        .about("Synchronize chorddown files with online file services")
        .subcommand(
            SubCommand::with_name("download")
                .version(env!("CARGO_PKG_VERSION"))
                .about("Download files from the service")
                .arg(
                    Arg::with_name("SERVICE")
                        .required(true)
                        .help("Online service to use (dropbox)"),
                )
                .arg(output_arg.clone())
                .arg(
                    Arg::with_name("API_TOKEN")
                        .long("api-key")
                        .takes_value(true)
                        .help("API key to authenticate with the service"),
                )
                .arg(
                    Arg::with_name("USERNAME")
                        .long("username")
                        .short("u")
                        .takes_value(true)
                        .help("Username to authenticate with the service"),
                )
                .arg(
                    Arg::with_name("PASSWORD")
                        .long("password")
                        .short("p")
                        .takes_value(true)
                        .help("Password to authenticate with the service"),
                )
                .arg(
                    Arg::with_name("URL")
                        .long("url")
                        .takes_value(true)
                        .help("WebDAV entry point URL"),
                )
                .arg(
                    Arg::with_name("REMOTE_DIRECTORY")
                        .long("remote-directory")
                        .takes_value(true)
                        .help("Remote directory to list"),
                ),
        )
        .get_matches();

    if let Err(error) = configure_logging(&args) {
        eprintln!("{}", error);
    }
    let error = if let Some(matches) = args.subcommand_matches("download") {
        download(matches)
    } else {
        eprintln!("Missing argument subcommand");
        Ok(())
    };

    if let Err(error) = error {
        eprintln!("{}", error)
    }
}

fn download(args: &ArgMatches) -> Result<()> {
    let service = get_service(args)?;

    let files = service.list_files()?;
    if files.len() == 0 {
        println!("No files found");
    }
    for file in &files {
        let destination = destination_for_file(&file.path(), args)?;
        if let Err(e) = check_if_should_download(file, &destination) {
            eprintln!("Skip download file {}: {}", file.path(), e)
        } else {
            match service.download(file.clone(), &destination) {
                Ok(_) => println!("Downloaded file {}", file.path()),
                Err(e) => eprintln!("Could not download file {}: {}", file.path(), e),
            }
        }
    }
    Ok(())
}

fn check_if_should_download(source: &FileEntry, destination: &Path) -> Result<()> {
    if !(destination.exists()) {
        return Ok(());
    }

    match destination.metadata() {
        Err(_) => Err(Error::download_error("Could not fetch metadata")),
        Ok(metadata) => match metadata.modified() {
            Err(_) => Err(Error::download_error("Could not fetch modification time")),
            Ok(modified) => {
                let remote_time = source.modified_date();
                let local_time: DateTime<Utc> = DateTime::from(modified);
                let local_time_utc = local_time.with_timezone(&remote_time.timezone());

                debug!(
                    "Compare remote vs local file time: {} vs {}",
                    remote_time, local_time_utc
                );
                if local_time_utc < remote_time {
                    info!("Remote file is newer than local file, will overwrite");
                    Ok(())
                } else {
                    Err(Error::download_error("Local file is newer than remote"))
                }
            }
        },
    }
}

fn get_api_key(args: &ArgMatches) -> Result<String> {
    if let Some(t) = args.value_of("API_TOKEN") {
        return Ok(t.to_owned());
    }

    match env::var("API_TOKEN") {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::missing_argument_error("No API token provided")),
    }
}

fn get_url(args: &ArgMatches) -> Result<String> {
    match args.value_of("URL") {
        Some(val) => Ok(val.to_owned()),
        None => Err(Error::missing_argument_error("No URL provided")),
    }
}

fn get_username(args: &ArgMatches) -> Result<String> {
    match args.value_of("USERNAME") {
        Some(val) => Ok(val.to_owned()),
        None => Err(Error::missing_argument_error("No username provided")),
    }
}

fn get_remote_directory(args: &ArgMatches) -> Result<String> {
    match args.value_of("REMOTE_DIRECTORY") {
        Some(val) => Ok(val.to_owned()),
        None => Err(Error::missing_argument_error(
            "No remote-directory provided",
        )),
    }
}

fn get_password(args: &ArgMatches) -> Result<String> {
    if let Some(t) = args.value_of("PASSWORD") {
        return Ok(t.to_owned());
    }

    match env::var("PASSWORD") {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::missing_argument_error("No password provided")),
    }
}

fn get_service(args: &ArgMatches) -> Result<Services> {
    let service_identifier = args.value_of("SERVICE").unwrap();
    match service_identifier.to_lowercase().as_str() {
        "dropbox" => Ok(Services::DropboxService(DropboxService::new(get_api_key(
            args,
        )?))),
        "webdav" => Ok(Services::WebDAVService(WebDAVService::new(
            get_url(args)?,
            get_remote_directory(args)?,
            get_username(args)?,
            get_password(args)?,
        )?)),
        _ => Err(Error::unknown_service_error(format!(
            "Service {} is not implemented",
            service_identifier
        ))),
    }
}

fn destination_for_file<P: AsRef<Path>>(file: &P, args: &ArgMatches) -> Result<PathBuf> {
    let output_path = PathBuf::from(args.value_of("OUTPUT").unwrap());
    let output_path_string = output_path.to_str().map_or_else(
        || format!("{}", output_path.to_string_lossy()),
        |s| s.to_owned(),
    );

    if output_path.is_dir() {
        match file.as_ref().file_name() {
            Some(file_name) => Ok(output_path.join(file_name)),
            None => Err(Error::io_error(format!(
                "Could not get remove name of file {}",
                file.as_ref().to_string_lossy()
            ))),
        }
    } else if output_path.is_file() {
        Err(Error::io_error(format!(
            "Output path {} is not a directory",
            output_path_string
        )))
    } else if !output_path.exists() {
        Err(Error::io_error(format!(
            "Output path {} does not exist",
            output_path_string
        )))
    } else {
        Err(Error::io_error(format!(
            "Output path {} is not a path",
            output_path_string
        )))
    }
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
