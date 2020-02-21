extern crate clap;
extern crate libchordr;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs;

use libchordr::models::chord::fmt::Formatting;
use libchordr::prelude::Error;
use libchordr::prelude::Result;
use libchordr::prelude::*;
use std::convert::TryFrom;

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
                .arg(Arg::with_name("FORMAT").help("Output format")),
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
        Ok(())
    };

    if let Err(error) = error {
        eprintln!("{}", error)
    }
}

fn convert(args: &ArgMatches) -> Result<()> {
    let input_file_path = args.value_of("INPUT").unwrap();
    let format = get_output_format(args);
    let contents = fs::read_to_string(input_file_path)?;
    let tokens = build_tokenizer().tokenize(&contents);
    let parser_result = Parser::new().parse(token_lines_to_tokens(tokens))?;
    let converted = Converter::new().convert(
        parser_result.node_as_ref(),
        parser_result.meta_as_ref(),
        Formatting::with_format(format),
    )?;

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

    handle_output(args, output)
}

fn get_output_format(args: &ArgMatches) -> Format {
    if let Some(raw_format) = args.value_of("FORMAT") {
        if let Ok(f) = Format::try_from(raw_format) {
            return f;
        }
    }

    Format::HTML
}

fn build_catalog(args: &ArgMatches) -> Result<()> {
    let dir_path = args.value_of("DIR").unwrap();
    let pretty = args.is_present("pretty");

    let catalog =
        CatalogBuilder::new().build_catalog_for_directory(dir_path, FileType::Chorddown, true)?;

    let serialization_result = if pretty {
        serde_json::to_string_pretty(&catalog)
    } else {
        serde_json::to_string(&catalog)
    };

    let output = match serialization_result {
        Ok(s) => s,
        Err(e) => return Err(Error::unknown_error(format!("{}", e))),
    };

    handle_output(args, output)
}

fn handle_output(args: &ArgMatches, output: String) -> Result<(), Error> {
    let output_file_path = args.value_of("OUTPUT").unwrap();
    if output_file_path == "-" {
        println!("{}", output);
        Ok(())
    } else {
        Ok(fs::write(output_file_path, output)?)
    }
}
