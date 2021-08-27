use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::Display;

/// Enum defining how the `B` is defined
///
/// In some european countries the `B` chord is written as `H`
#[derive(Deserialize, Serialize, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum BNotation {
    B,
    H,
}

impl BNotation {
    pub fn is_european_chord(chord: &str) -> bool {
        matches!(Self::try_from(chord), Ok(Self::H))
    }

    pub fn contains_european_chord(chord: &str) -> bool {
        chord.contains('H') || chord.contains('h')
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::B => "B",
            Self::H => "H",
        }
    }
}

impl Default for BNotation {
    fn default() -> Self {
        Self::B
    }
}

impl Display for BNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<char> for BNotation {
    type Error = NotationError;

    //noinspection RsTypeCheck
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'B' | 'b' => Ok(Self::B),
            'H' | 'h' => Ok(Self::H),
            _ => Err(NotationError(value.to_string())),
        }
    }
}

impl TryFrom<&str> for BNotation {
    type Error = NotationError;

    //noinspection RsTypeCheck
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "B" | "b" => Ok(Self::B),
            "H" | "h" => Ok(Self::H),
            _ => Err(NotationError(value.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct NotationError(String);

impl std::error::Error for NotationError {}

impl Display for NotationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Invalid b-notation '{}'", self.0)
    }
}
