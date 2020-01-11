use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

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
        match Self::try_from(chord) {
            Ok(Self::H) => true,
            _ => false
        }
    }
}

impl Default for BNotation {
    fn default() -> Self {
        Self::B
    }
}

pub struct NotationError {}

impl TryFrom<char> for BNotation {
    type Error = NotationError;

    //noinspection RsTypeCheck
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'B' | 'b' => Ok(Self::B),
            'H' | 'h' => Ok(Self::H),
            _ => Err(NotationError {})
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
            _ => Err(NotationError {})
        }
    }
}
