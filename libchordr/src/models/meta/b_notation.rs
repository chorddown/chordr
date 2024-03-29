use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<char> for BNotation {
    type Error = NotationError;

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

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl FromStr for BNotation {
    type Err = NotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid b-notation '{}'", self.0)
    }
}
