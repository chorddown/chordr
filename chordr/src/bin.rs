extern crate clap;
extern crate libchordr;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs;

use ansi_term::Colour;
use atty::Stream;
use libchordr::models::chord::fmt::Formatting;
use libchordr::prelude::Error;
use libchordr::prelude::Result;
use libchordr::prelude::*;
use std::convert::TryFrom;
use std::error::Error as StdError;
use std::process::exit;

fn main() {
    let output_arg = Arg::with_name("OUTPUT")
        .required(true)
        .help("Output file name");
    let args = App::new("chordr")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daniel Corn <info@cundd.net>")
        .about("Manage chorddown files and catalogs")
        .subcommand(
            SubCommand::with_name("convert")
                .about("Convert chorddown files")
                .arg(
                    Arg::with_name("INPUT")
                        .required(true)
                        .help("Chorddown file to parse"),
                )
                .arg(output_arg.clone())
                .arg(Arg::with_name("FORMAT").help(&get_output_format_help())),
        )
        .subcommand(
            SubCommand::with_name("build-catalog")
                .about("Build a catalog from chorddown files")
                .arg(
                    Arg::with_name("DIR")
                        .required(true)
                        .help("Path to the directory of chorddown files"),
                )
                .arg(output_arg.clone())
                .arg(
                    Arg::with_name("pretty")
                        .long("pretty")
                        .short("p")
                        .help("Output indented JSON"),
                ),
        )
        .get_matches();

    let error = if let Some(matches) = args.subcommand_matches("convert") {
        convert(matches)
    } else if let Some(matches) = args.subcommand_matches("build-catalog") {
        build_catalog(matches)
    } else {
        eprintln!("Missing argument subcommand");
        exit(1);
    };

    if let Err(error) = error {
        eprintln!("{}", error);
        exit(1);
    }
}

fn convert(args: &ArgMatches) -> Result<()> {
    let input_file_path = args.value_of("INPUT").unwrap();
    let format = get_output_format(args);
    let contents = fs::read_to_string(input_file_path)?;
    let tokens = build_tokenizer().tokenize(&contents);
    let parser_result = Parser::new().parse(tokens)?;
    let converted = Converter::new().convert(
        parser_result.node_as_ref(),
        parser_result.meta_as_ref(),
        Formatting::with_format(format),
    )?;
    let output_file_path = args.value_of("OUTPUT").unwrap();

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
            title = parser_result.meta().title.unwrap_or("".to_owned()),
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
    if let Some(raw_format) = args.value_of("FORMAT") {
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
    let dir_path = args.value_of("DIR").unwrap();
    let pretty = args.is_present("pretty");
    let output_file_path = args.value_of("OUTPUT").unwrap();

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
        Ok(fs::write(output_file_path, output)?)
    }
}

fn output_to_stdout(output_file_path: &str) -> bool {
    output_file_path == "-"
}
