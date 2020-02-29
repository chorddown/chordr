mod chorddown_tokenizer;
mod meta;
mod modifier;
mod token;

use self::chorddown_tokenizer::ChorddownTokenizer;
pub use self::meta::Meta;
pub use self::modifier::Modifier;
pub use self::token::Token;
//use crate::tokenizer::chord_pro_tokenizer::ChordProTokenizer;

//#[deprecated(note = "Please use the `Token`s directly")]
pub type TokenLine = Vec<Token>;

pub trait Tokenizer {
    /// Tokenize the given input
    ///
    /// The input will be split into individual lines. These lines will then be sent to
    /// `tokenize_line()`. Each sent line will contain a trailing newline (`\n`)
    fn tokenize(&self, input: &str) -> Vec<Token>;
}

/// Build a new Tokenizer instance
pub fn build_tokenizer() -> impl Tokenizer {
    ChorddownTokenizer::new()
}
