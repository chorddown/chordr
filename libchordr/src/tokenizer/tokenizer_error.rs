use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum TokenizerError {
    UnclosedChord,
    NestedChord,
    InvalidChordCharacter,
    UnexpectedChordEnd,
    UnexpectedHeaderStart,
    UnexpectedEndOfFile,
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenizerError::UnclosedChord => f.write_str("UnclosedChord"),
            TokenizerError::NestedChord => f.write_str("NestedChord"),
            TokenizerError::InvalidChordCharacter => f.write_str("InvalidChordCharacter"),
            TokenizerError::UnexpectedChordEnd => f.write_str("UnexpectedChordEnd"),
            TokenizerError::UnexpectedHeaderStart => f.write_str("UnexpectedHeaderStart"),
            TokenizerError::UnexpectedEndOfFile => f.write_str("UnexpectedEndOfFile"),
        }
    }
}

impl Error for TokenizerError {}
