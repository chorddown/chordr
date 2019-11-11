mod token;
//mod meta;
mod chord_down_tokenizer;

pub use self::token::Token;
use self::chord_down_tokenizer::ChordDownTokenizer;
//pub use self::directive::Directive;
//use crate::tokenizer::chord_pro_tokenizer::ChordProTokenizer;

pub type TokenLine = Vec<Token>;

pub trait Tokenizer {
    fn tokenize(&self, input: &str) -> Vec<TokenLine> {
        let mut token_lines: Vec<TokenLine> = vec![];
        for line in input.lines() {
            token_lines.push(self.tokenize_line(&(line.to_owned() + "\n")))
        }
        token_lines
    }

    fn tokenize_line(&self, line: &str) -> Vec<Token>;
}

/// Build a new Tokenizer instance
pub fn build_tokenizer() -> impl Tokenizer {
    ChordDownTokenizer::new()
}
