use crate::configuration::Configuration;
use crate::error::{Error, Result};
use crate::task::{BuildCatalogTask, CollectionTask, DownloadTask, RecurringTaskTrait, TaskTrait};
use clap::{App, Arg, ArgMatches};
use configuration::reader::Reader;
use log::{error, info};
use simplelog::{ColorChoice, Config, TerminalMode};
use std::env;
use std::path::Path;
use std::process::exit;
use std::{thread, time};

mod configuration;
mod error;
mod task;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run() -> Result<()> {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daniel Corn <info@cundd.net>")
        .about("Service for chorddown file synchronization and catalog building")
        .arg(
            Arg::with_name("configuration")
                .help("File containing the configuration")
                .short("c")
                .long("configuration")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pretty")
                .help("Output indented JSON")
                .short("p")
                .long("pretty"),
        )
        .arg(
            Arg::with_name("verbosity")
                .help("Change the verbosity of the output")
                .short("v")
                .multiple(true),
        );
    let matches = app.get_matches();
    configure_logging(&matches)?;
    let configuration = read_configuration(&matches)?;

    start_loop(&configuration)
}

fn start_loop(configuration: &Configuration) -> Result<()> {
    let sleep_interval = time::Duration::from_secs(configuration.service.sync_interval);
    let download_task = DownloadTask::with_configuration(configuration.clone())?;
    let build_catalog_task = BuildCatalogTask::with_configuration(configuration.clone())?;

    let collection_task = CollectionTask::new(vec![&download_task, &build_catalog_task]);
    info!(
        "Start task loop with an interval of {} seconds",
        configuration.service.sync_interval
    );
    loop {
        info!("Run tasks");
        if let Err(e) = collection_task.run() {
            error!("{}", e);
        }
        thread::sleep(sleep_interval);
    }
}

fn read_configuration(args: &ArgMatches<'_>) -> Result<Configuration> {
    Reader::read_configuration_from_file(Path::new(args.value_of("configuration").unwrap()))
}

fn configure_logging(matches: &ArgMatches<'_>) -> Result<()> {
    let log_level_filter = match matches.occurrences_of("verbosity") {
        1 => simplelog::LevelFilter::Info,
        2 => simplelog::LevelFilter::Debug,
        3 => simplelog::LevelFilter::Trace,
        _ => simplelog::LevelFilter::Error,
    };

    simplelog::TermLogger::init(
        log_level_filter,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .map_err(|e| Error::io_error(e.to_string()))
}
