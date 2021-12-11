use std::io::BufRead;

use crate::converter::{Converter, ConverterTrait};
use crate::error::Result;
use crate::metadata::metadata_trait::MetadataTrait;
use crate::models::chord::fmt::Formatting;
use crate::models::chord::TransposableTrait;
use crate::parser::{Parser, ParserResult, ParserTrait};
use crate::tokenizer::{build_tokenizer, Token, Tokenizer};

#[deprecated(note = "Please use the `Token`s directly")]
pub fn token_lines_to_tokens(token_lines: Vec<Vec<Token>>) -> Vec<Token> {
    let mut stream = vec![];
    for line in token_lines {
        for token in line {
            stream.push(token);
        }
    }

    stream
}

pub fn parse_content<R: BufRead>(contents: R) -> Result<ParserResult> {
    let tokens = build_tokenizer().tokenize(contents)?;
    Parser::new().parse(tokens)
}

pub fn transpose_content<R: BufRead>(contents: R, semitones: isize) -> Result<ParserResult> {
    let ParserResult { node, metadata } = parse_content(contents)?;

    let transposed_node = node.transpose(semitones);
    let transposed_meta = metadata.transpose(semitones);

    Ok(ParserResult::new(transposed_node, transposed_meta))
}

pub fn convert_to_format<R: BufRead>(
    contents: R,
    metadata: &dyn MetadataTrait,
    formatting: Formatting,
) -> Result<String> {
    Converter::new().convert(parse_content(contents)?.node(), metadata, formatting)
}

pub fn transpose_and_convert_to_format<R: BufRead>(
    contents: R,
    semitones: isize,
    _metadata: &dyn MetadataTrait,
    formatting: Formatting,
) -> Result<String> {
    let ParserResult { metadata, node } = transpose_content(contents, semitones)?;

    Converter::new().convert(&node, &metadata, formatting)
}

#[allow(unused)]
pub(crate) fn is_valid_model_identifier(id: &str) -> bool {
    validate_model_identifier(id).is_ok()
}

pub(crate) fn validate_model_identifier(id: &str) -> Result<(), &'static str> {
    if id.is_empty() {
        return Err("Identifier must not be empty");
    }
    if !id.is_ascii() {
        return Err("Identifier must not be an ASCII text");
    }
    fn is_char_allowed(c: char) -> bool {
        c.is_alphanumeric() || c == '-' || c == '_' || c == '@' || c == '.'
    }
    if id.chars().all(is_char_allowed) {
        Ok(())
    } else {
        Err("Identifier can only contain alphanumerics, dash ('-'), underscore ('_'), @ or a dot ('.')")
    }
}
