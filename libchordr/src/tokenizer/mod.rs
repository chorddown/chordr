mod chorddown_tokenizer;
mod meta;
mod modifier;
mod token;

use self::chorddown_tokenizer::ChorddownTokenizer;
pub use self::meta::Meta;
pub use self::modifier::Modifier;
pub use self::token::Token;
//pub use self::directive::Directive;
//use crate::tokenizer::chord_pro_tokenizer::ChordProTokenizer;

pub type TokenLine = Vec<Token>;

pub trait Tokenizer {
    /// Tokenize the given input
    ///
    /// The input will be split into individual lines. These lines will then be sent to
    /// `tokenize_line()`. Each sent line will contain a trailing newline (`\n`)
    fn tokenize(&self, input: &str) -> Vec<TokenLine> {
        let mut token_lines: Vec<TokenLine> = vec![];
        for line in input.lines() {
            let token_line = self.tokenize_line(&(line.to_owned() + "\n"));
            if let Some(token_line) = token_line {
                token_lines.push(token_line);
            }
        }
        token_lines
    }

    /// Tokenize an individual line
    fn tokenize_line(&self, line: &str) -> Option<Vec<Token>>;
}

/// Build a new Tokenizer instance
pub fn build_tokenizer() -> impl Tokenizer {
    ChorddownTokenizer::new()
}
