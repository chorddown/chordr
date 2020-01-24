use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};

/// Enum defining if `C#` or `Db` should be used
#[derive(Deserialize, Serialize, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum SemitoneNotation {
    Sharp,
    Flat,
}

impl SemitoneNotation {}

impl Default for SemitoneNotation {
    fn default() -> Self {
        Self::Sharp
    }
}

pub struct SemitoneNotationError {}

impl TryFrom<char> for SemitoneNotation {
    type Error = SemitoneNotationError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' | '♯' => Ok(Self::Sharp),
            'b' | '♭' => Ok(Self::Flat),
            _ => Err(SemitoneNotationError {})
        }
    }
}

impl TryFrom<&str> for SemitoneNotation {
    type Error = SemitoneNotationError;

    //noinspection RsTypeCheck
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "#" | "♯" => Ok(Self::Sharp),
            "b" | "♭" => Ok(Self::Flat),
            _ => Err(SemitoneNotationError {})
        }
    }
}

impl Display for SemitoneNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self {
            Self::Sharp => "♯",
            Self::Flat => "♭",
        })
    }
}

