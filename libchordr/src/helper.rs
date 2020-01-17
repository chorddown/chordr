use crate::converter::{Converter, ConverterTrait};
use crate::error::Result;
use crate::format::Format;
use crate::parser::{Parser, ParserTrait, ParserResult};
use crate::tokenizer::{build_tokenizer, Token, TokenLine, Tokenizer};
use crate::models::song_meta_trait::SongMetaTrait;
use crate::models::chord::TransposableTrait;

pub fn token_lines_to_tokens(token_lines: Vec<TokenLine>) -> Vec<Token> {
    let mut stream = vec![];
    for line in token_lines {
        for token in line {
            stream.push(token);
        }
    }

    stream
}

pub fn parse_content(contents: &str) -> Result<ParserResult> {
    let tokens = build_tokenizer().tokenize(contents);
    Parser::new().parse(token_lines_to_tokens(tokens))
}

pub fn transpose_content(contents: &str, semitones: isize) -> Result<ParserResult> {
    let parser_result = parse_content(contents)?;
    let transposed_node = parser_result.node_as_ref().transpose(semitones);

    Ok(ParserResult::new(transposed_node, parser_result.meta()))
}

pub fn convert_to_format(contents: &str, meta: &dyn SongMetaTrait, format: Format) -> Result<String> {
    Converter::new().convert(parse_content(contents)?.node_as_ref(), meta, format)
}

pub fn transpose_and_convert_to_format(contents: &str, semitones: isize, meta: &dyn SongMetaTrait, format: Format) -> Result<String> {
    Converter::new().convert(transpose_content(contents, semitones)?.node_as_ref(), meta, format)
}
