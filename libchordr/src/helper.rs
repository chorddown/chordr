use crate::converter::{Converter, ConverterTrait};
use crate::error::Result;
use crate::format::Format;
use crate::parser::{Parser, ParserResult};
use crate::tokenizer::{build_tokenizer, Token, TokenLine, Tokenizer};

pub fn token_lines_to_tokens(token_lines: Vec<TokenLine>) -> Vec<Token> {
    let mut stream = vec![];
    for line in token_lines {
        for token in line {
            stream.push(token);
        }
    }

    stream
}

pub fn parse_content(contents: &str) -> ParserResult {
    let tokens = build_tokenizer().tokenize(contents);
    Parser::new().parse(token_lines_to_tokens(tokens))
}

pub fn convert_to_format(contents: &str, format: Format) -> Result<String> {
    Converter::new().convert(parse_content(contents).node_as_ref(), format)
}
