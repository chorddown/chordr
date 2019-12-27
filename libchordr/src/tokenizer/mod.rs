mod modifier;
mod token;
//mod meta;
mod chord_down_tokenizer;

use self::chord_down_tokenizer::ChordDownTokenizer;
pub use self::modifier::Modifier;
pub use self::token::Token;
//pub use self::directive::Directive;
//use crate::tokenizer::chord_pro_tokenizer::ChordProTokenizer;

pub type TokenLine = Vec<Token>;

pub trait Tokenizer {
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

    fn tokenize_line(&self, line: &str) -> Option<Vec<Token>>;
}

/// Build a new Tokenizer instance
pub fn build_tokenizer() -> impl Tokenizer {
    ChordDownTokenizer::new()
}
