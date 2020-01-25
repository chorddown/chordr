extern crate clap;

use crate::error::{Error, Result};
use crate::service::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::env;
use std::path::{PathBuf, Path};

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
                ),
        )
        .get_matches();

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
        match service.download(file.clone(), &destination_for_file(&file, args)?) {
            Ok(_) => println!("Downloaded file {}", file),
            Err(e) => eprintln!("Could not download file {}: {}", file, e)
        }
    }
    Ok(())
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

fn get_service(args: &ArgMatches) -> Result<Services> {
    let service_identifier = args.value_of("SERVICE").unwrap();
    match service_identifier.to_lowercase().as_str() {
        "dropbox" => Ok(Services::DropboxService(DropboxService::new(get_api_key(args)?))),
        _ => Err(Error::unknown_service_error(format!("Service {} is not implemented", service_identifier))),
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
            None => Err(Error::io_error(format!("Could not get remove name of file {}", file.as_ref().to_string_lossy())))
        }
    } else if output_path.is_file() {
        Err(Error::io_error(format!("Output path {} is not a directory", output_path_string)))
    } else if !output_path.exists() {
        Err(Error::io_error(format!("Output path {} does not exist", output_path_string)))
    } else {
        Err(Error::io_error(format!("Output path {} is not a path", output_path_string)))
    }
}
