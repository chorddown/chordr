extern crate clap;
extern crate libchordr;

use std::convert::TryFrom;
use std::error::Error as StdError;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;

use ansi_term::Colour;
use atty::Stream;
use clap::{App, Arg, ArgMatches, SubCommand};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

use libchordr::models::chord::fmt::Formatting;
use libchordr::models::chord::TransposableTrait;
use libchordr::prelude::Error;
use libchordr::prelude::Result;
use libchordr::prelude::*;

fn main() {
    let output_arg = Arg::with_name("output")
        .required(true)
        .help("Output file name");
    let verbosity_arg = Arg::with_name("verbosity")
        .short("v")
        .multiple(true)
        .help("Set the output verbosity");

    let format_help = get_output_format_help();
    let subcommand_convert = SubCommand::with_name("convert")
        .about("Convert chorddown files")
        .arg(
            Arg::with_name("input")
                .required(true)
                .help("Chorddown file to parse"),
        )
        .arg(output_arg.clone())
        .arg(Arg::with_name("format").help(&format_help))
        .arg(
            Arg::with_name("transpose")
                .long("transpose")
                .takes_value(true)
                .help("Number of semitones to transpose the song"),
        )
        .arg(
            Arg::with_name("b-notation")
                .long("b-notation")
                .takes_value(true)
                .help("Define how the `B` chord will be displayed"),
        )
        .arg(
            Arg::with_name("semitone-notation")
                .long("semitone-notation")
                .takes_value(true)
                .help("Define if `#` or `b` should be used"),
        )
        .arg(verbosity_arg.clone());

    let subcommand_build_catalog = SubCommand::with_name("build-catalog")
        .about("Build a catalog from chorddown files")
        .arg(
            Arg::with_name("dir")
                .required(true)
                .help("Path to the directory of chorddown files"),
        )
        .arg(output_arg.clone())
        .arg(
            Arg::with_name("pretty")
                .long("pretty")
                .short("p")
                .help("Output indented JSON"),
        )
        .arg(verbosity_arg);

    let args = App::new("chordr")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daniel Corn <info@cundd.net>")
        .about("Manage chorddown files and catalogs")
        .subcommand(subcommand_convert)
        .subcommand(subcommand_build_catalog)
        .get_matches();

    if let Err(error) = run(args) {
        eprintln!("{}", error);
        exit(1);
    }
}

fn run(args: ArgMatches) -> Result<()> {
    if let Some(matches) = args.subcommand_matches("convert") {
        configure_logging(matches)?;
        convert(matches)
    } else if let Some(matches) = args.subcommand_matches("build-catalog") {
        configure_logging(matches)?;
        build_catalog(matches)
    } else {
        eprintln!("Missing argument subcommand");
        exit(1);
    }
}

fn convert(args: &ArgMatches) -> Result<()> {
    let input_file_path = args.value_of("input").unwrap();
    let output_file_path = args.value_of("output").unwrap();

    let b_notation = match args.value_of("b-notation") {
        None => BNotation::default(),
        Some(b) => BNotation::try_from(b)?,
    };
    let semitone_notation = match args.value_of("semitone-notation") {
        None => SemitoneNotation::default(),
        Some(b) => SemitoneNotation::try_from(b)?,
    };

    let transpose: Option<isize> = match args.value_of("transpose") {
        None => None,
        Some(raw) => Some(str::parse::<isize>(raw)?),
    };

    let format = get_output_format(args);

    let formatting = Formatting {
        b_notation,
        semitone_notation,
        format,
    };

    let file = match File::open(input_file_path) {
        Ok(c) => c,
        Err(e) => return Err(Error::unknown_error(format!("Could not read file: {}", e))),
    };
    log::debug!("Did read file");

    let tokens = build_tokenizer().tokenize(BufReader::new(file))?;
    log::debug!("Did tokenize content");

    let parser_result = Parser::new().parse(tokens)?;
    log::debug!("Did parse content");
    let meta = parser_result.meta_as_ref().clone();
    let parser_result_node = match transpose {
        None => parser_result.node(),
        Some(t) => parser_result.node().transpose(t),
    };

    let converted = Converter::new().convert(&parser_result_node, &meta, formatting)?;
    log::debug!("Did convert content");

    let output = if format == Format::HTML {
        format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{title}</title>
    <style>{styles}</style>
</head>
<body>
<main>
{content}
</main>
</body>
</html>
    "#,
            title = meta.title.unwrap_or("".to_owned()),
            styles = include_str!("../../webchordr/static/stylesheets/chordr-default-styles.css"),
            content = converted
        )
    } else {
        converted
    };

    handle_output(output_file_path, output)
}

fn get_output_format_help() -> String {
    format!("Output format (one of {})", get_valid_output_format_help())
}

fn get_valid_output_format_help() -> String {
    Format::get_all()
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn get_output_format(args: &ArgMatches) -> Format {
    if let Some(raw_format) = args.value_of("format") {
        match Format::try_from(raw_format) {
            Ok(f) => return f,
            Err(_) => {
                eprintln!(
                    "Unknown format '{}'. Valid formats are: {}",
                    raw_format,
                    get_valid_output_format_help()
                );
                exit(1);
            }
        }
    }

    Format::HTML
}

fn build_catalog(args: &ArgMatches) -> Result<()> {
    let dir_path = args.value_of("dir").unwrap();
    let pretty = args.is_present("pretty");
    let output_file_path = args.value_of("output").unwrap();

    let catalog_result =
        CatalogBuilder::new().build_catalog_for_directory(dir_path, FileType::Chorddown, true)?;

    let serialization_result = if pretty {
        serde_json::to_string_pretty(&catalog_result.catalog)
    } else {
        serde_json::to_string(&catalog_result.catalog)
    };

    let output = match serialization_result {
        Ok(s) => s,
        Err(e) => return Err(Error::unknown_error(format!("{}", e))),
    };
    if catalog_result.errors.len() > 0 {
        for error in catalog_result.errors {
            handle_error_output(error)
        }
    }

    handle_output(output_file_path, output)?;

    if !output_to_stdout(output_file_path) {
        let msg = format!(
            "Successfully saved the catalog revision '{}' at {}",
            catalog_result.catalog.revision(),
            output_file_path
        );
        if atty::is(Stream::Stdout) {
            println!("{}", Colour::Green.paint(msg));
        } else {
            println!("{}", msg);
        }
    }
    Ok(())
}

fn configure_logging(matches: &ArgMatches) -> Result<()> {
    let level_filter = match matches.occurrences_of("verbosity") {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 | _ => LevelFilter::Trace,
    };

    match CombinedLogger::init(vec![TermLogger::new(
        level_filter,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )]) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::unknown_error(format!(
            "Could not initialize logger: {}",
            e
        ))),
    }
}

fn handle_error_output(error: CatalogBuildError) -> () {
    let header = format!(
        "Error during analysis of file {}:",
        error.path().to_string_lossy()
    );
    let description = match error.source() {
        Some(s) => s.to_string(),
        None => error.message().to_owned(),
    };

    if atty::is(Stream::Stderr) {
        eprintln!("{}", Colour::White.on(Colour::Red).paint(header));
        eprintln!("{}", Colour::Red.paint(description));
    } else {
        eprintln!("{}", header);
        eprintln!("{}", description);
    }
}

fn handle_output(output_file_path: &str, output: String) -> Result<(), Error> {
    if output_to_stdout(output_file_path) {
        println!("{}", output);
        Ok(())
    } else {
        match fs::write(output_file_path, output) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::unknown_error(format!("Could not write file: {}", e))),
        }
    }
}

fn output_to_stdout(output_file_path: &str) -> bool {
    output_file_path == "-"
}
