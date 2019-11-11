extern crate clap;
extern crate libchordr;

use clap::{Arg, App};
use std::fs;

use libchordr::prelude::*;
use libchordr::prelude::build_tokenizer;

fn main() {
    let args = App::new("chordr")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daniel Corn <info@cundd.net>")
        .about("Convert chordpro files")
        .arg(Arg::with_name("OUTPUT")
            .long("output")
            .short("o")
            .takes_value(true)
            .help("Output file name"))
        // .arg(Arg::with_name("TITLE")
        //     .long("title")
        //     .takes_value(true)
        //     .help("Title (in metadata) of the output file"))
        // .arg(Arg::with_name("AUTHOR")
        //     .long("author")
        //     .takes_value(true)
        //     .help("Author (in metadata) of the output file"))
        // .arg(Arg::with_name("SOURCENAMES")
        //     .long("sourcenames")
        //     .help("Show name of chopro source file on page"))
        // .arg(Arg::with_name("CHORDS")
        //     .long("chords")
        //     .help("Add a separate page of chord definitions"))
        .arg(Arg::with_name("INPUT")
            .required(true)
            .multiple(true)
            .help("Chopro file(s) to parse"))
        // .after_help(
        //     "At least one INPUT file is required unless the --chords \
        //      flag is given.\n\n\
        //      Each INPUT file contains one or more song in the \
        //      chopro format, which is described at \
        //      https://github.com/kaj/chord3/blob/master/chopro.md .",
        // )
        .get_matches();

    let file_path = args.value_of("INPUT").unwrap();
    let contents = fs::read_to_string(file_path)
        .expect(&format!("File '{}' could not be read", file_path));

    let tokens = build_tokenizer().tokenize(&contents);

    let converted = Converter::new().convert(&tokens, Format::HTML).unwrap();

    println!(
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
        title = "",
        styles = include_str!("../resources/html/main.css"),
        content = converted);
}
