use std::io::BufRead;

use crate::error::Error;

use self::chorddown_tokenizer::ChorddownTokenizer;
pub use self::meta::Meta;
pub use self::modifier::Modifier;
pub use self::token::Token;

mod chorddown_tokenizer;
mod meta;
mod modifier;
mod token;

pub trait Tokenizer {
    /// Tokenize the given input
    fn tokenize<R: BufRead>(&self, input: R) -> Result<Vec<Token>, Error>;
}

/// Build a new Tokenizer instance
pub fn build_tokenizer() -> impl Tokenizer {
    ChorddownTokenizer::new()
}
