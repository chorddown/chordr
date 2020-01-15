use crate::error::Error;
use crate::models::meta::BNotation;
use std::fmt::Display;
use serde::export::Formatter;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Note {
    Cis,
    C,
    Dis,
    D,
    Eis,
    E,
    Fis,
    F,
    Gis,
    G,
    Ais,
    A,
    B,
}

impl Note {
    pub fn to_string(&self, b_notation: BNotation) -> &str {
        match self {
            Self::Cis => "C#",
            Self::C => "C",
            Self::Dis => "D#",
            Self::D => "D",
            Self::Eis => "E#",
            Self::E => "E",
            Self::Fis => "F#",
            Self::F => "F",
            Self::Gis => "G#",
            Self::G => "G",
            Self::Ais => "A#",
            Self::A => "A",
            Self::B => if b_notation == BNotation::B { "B" } else { "H" },
        }
    }

    pub fn try_from(value: &str, b_notation: BNotation) -> Result<Self, Error> {
        if value.is_empty() {
            return Err(Error::chord_error("Given note is empty"));
        }

        match value.to_lowercase().as_str() {
            // H
            "h" => Ok(Self::B),

            // B
            // The meaning of "B" depends on the song's B-Notation
            "b" => {
                if b_notation == BNotation::B {
                    Ok(Self::B)
                } else {
                    Ok(Self::Ais)
                }
            }

            // A#
            "a#"
            | "a♯"
            // | "ais"
            | "bb"
            | "b♭"
            | "h♭"
            => Ok(Self::Ais),

            // A
            "a" => Ok(Self::A),

            // G#
            "g#"
            | "g♯"
            // | "gis"
            | "ab"
            | "a♭"
            // | "as"
            => Ok(Self::Gis),

            // G
            "g" => Ok(Self::G),

            // F#
            "f#"
            | "f♯"
            // | "fis"
            | "gb"
            | "g♭"
            // | "ges"
            => Ok(Self::Fis),

            // F
            "f" => Ok(Self::F),

            // E
            "e" => Ok(Self::E),

            // D#
            "d#"
            | "d♯"
            // | "dis"
            | "eb"
            | "e♭"
            // | "es"
            => Ok(Self::Dis),

            // D
            "d" => Ok(Self::D),

            // C#
            "c#"
            | "c♯"
            // | "cis"
            | "db"
            | "d♭"
            // | "des"
            => Ok(Self::Cis),

            // C
            "c" => Ok(Self::C),

            _ => return Err(Error::chord_error(format!("Unknown note {}", value)))
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", match self {
            Self::Cis => "C#",
            Self::C => "C",
            Self::Dis => "D#",
            Self::D => "D",
            Self::Eis => "E#",
            Self::E => "E",
            Self::Fis => "F#",
            Self::F => "F",
            Self::Gis => "G#",
            Self::G => "G",
            Self::Ais => "A#",
            Self::A => "A",
            Self::B => "B",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from() {
        assert_eq!(Note::try_from("A", BNotation::B).unwrap(), Note::A);
        assert_eq!(Note::try_from("A#", BNotation::B).unwrap(), Note::Ais);

        assert_eq!(Note::try_from("C#", BNotation::H).unwrap(), Note::Cis);

        assert_eq!(Note::try_from("Bb", BNotation::B).unwrap(), Note::Ais);
        assert_eq!(Note::try_from("bb", BNotation::B).unwrap(), Note::Ais);

        assert_eq!(Note::try_from("B", BNotation::B).unwrap(), Note::B);
        assert_eq!(Note::try_from("B", BNotation::H).unwrap(), Note::Ais);

        assert_eq!(Note::try_from("H♭", BNotation::B).unwrap(), Note::Ais);
        assert_eq!(Note::try_from("H♭", BNotation::H).unwrap(), Note::Ais);
        assert_eq!(Note::try_from("B", BNotation::H).unwrap(), Note::Ais);
        assert!(Note::try_from("H#", BNotation::H).is_err());

        assert_eq!(Note::try_from("F", BNotation::B).unwrap(), Note::F);
        assert_eq!(Note::try_from("F#", BNotation::B).unwrap(), Note::Fis);
        assert_eq!(Note::try_from("F♯", BNotation::B).unwrap(), Note::Fis);
        assert!(Note::try_from("Fb", BNotation::H).is_err());

        assert!(Note::try_from("X", BNotation::H).is_err());
    }
}
