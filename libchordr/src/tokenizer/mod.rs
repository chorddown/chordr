use std::io::BufRead;

use crate::error::Error;

use self::chorddown_tokenizer::ChorddownTokenizer;
pub use self::meta::Meta;
pub use self::modifier::Modifier;
pub use self::token::Token;
use self::tokenizer_error::TokenizerError;

mod chorddown_tokenizer;
mod meta;
mod modifier;
mod token;
mod tokenizer_error;

pub trait Tokenizer {
    /// Tokenize the given input
    fn tokenize<R: BufRead>(&self, input: R) -> Result<(Vec<Token>, Vec<TokenizerError>), Error>;
}

/// Build a new Tokenizer instance
pub fn build_tokenizer() -> impl Tokenizer {
    ChorddownTokenizer::new()
}
