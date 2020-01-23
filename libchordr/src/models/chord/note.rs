use crate::error::Error;
use crate::models::meta::BNotation;
use std::fmt::Debug;
use serde::export::Formatter;
use crate::models::chord::transposition::TransposableTrait;
use crate::models::chord::NoteDisplay;
use crate::models::chord::fmt::Formatting;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Note {
    C = 1,
    Cis,
    D,
    Dis,
    E,
    F,
    Fis,
    G,
    Gis,
    A,
    Ais,
    B,
}

impl Note {
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

impl Debug for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", match self {
            Self::C => "C",
            Self::Cis => "C#",
            Self::D => "D",
            Self::Dis => "D#",
            Self::E => "E",
            Self::F => "F",
            Self::Fis => "F#",
            Self::G => "G",
            Self::Gis => "G#",
            Self::A => "A",
            Self::Ais => "A#",
            Self::B => "B",
        })
    }
}

impl NoteDisplay for Note {
    fn to_string(&self, formatting: Formatting) -> String {
        match self {
            Self::C => "C",
            Self::Cis => "C#",
            Self::D => "D",
            Self::Dis => "D#",
            Self::E => "E",
            Self::F => "F",
            Self::Fis => "F#",
            Self::G => "G",
            Self::Gis => "G#",
            Self::A => "A",
            Self::Ais => "A#",
            Self::B => if formatting.b_notation == BNotation::B { "B" } else { "H" },
        }.to_string()
    }
}

impl From<Note> for i32 {
    fn from(n: Note) -> Self {
        n as i32
    }
}

impl From<&Note> for i32 {
    fn from(n: &Note) -> Self {
        *n as i32
    }
}

impl From<isize> for Note {
    fn from(d: isize) -> Self {
        let scaled = d % 12;

        match scaled {
            0 => Note::B,
            1 => Note::C,
            2 => Note::Cis,
            3 => Note::D,
            4 => Note::Dis,
            5 => Note::E,
            6 => Note::F,
            7 => Note::Fis,
            8 => Note::G,
            9 => Note::Gis,
            10 => Note::A,
            11 => Note::Ais,

            _ if scaled < 1 => {
                Note::from(12 + scaled)
            }
            _ => unreachable!()
        }
    }
}

impl TransposableTrait for Note {
    fn transpose(&self, semitones: isize) -> Self {
        Note::from(*self as isize + semitones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_test() {
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

        assert_eq!(Note::try_from("F", BNotation::B).unwrap(), Note::F);
        assert_eq!(Note::try_from("F#", BNotation::B).unwrap(), Note::Fis);
        assert_eq!(Note::try_from("F♯", BNotation::B).unwrap(), Note::Fis);

        assert!(Note::try_from("X", BNotation::H).is_err());
        assert!(Note::try_from("Eis", BNotation::H).is_err());
        assert!(Note::try_from("E#", BNotation::H).is_err());
        assert!(Note::try_from("His", BNotation::H).is_err());
        assert!(Note::try_from("H#", BNotation::H).is_err());
        assert!(Note::try_from("Cb", BNotation::H).is_err());
        assert!(Note::try_from("Fb", BNotation::H).is_err());
    }

    #[test]
    fn as_isize_test() {
        assert_eq!(Note::C as isize, 1, "`as isize` failed for Note C");
        assert_eq!(Note::Cis as isize, 2, "`as isize` failed for Note Cis");
        assert_eq!(Note::D as isize, 3, "`as isize` failed for Note D");
        assert_eq!(Note::Dis as isize, 4, "`as isize` failed for Note Dis");
        assert_eq!(Note::E as isize, 5, "`as isize` failed for Note E");
        assert_eq!(Note::F as isize, 6, "`as isize` failed for Note F");
        assert_eq!(Note::Fis as isize, 7, "`as isize` failed for Note Fis");
        assert_eq!(Note::G as isize, 8, "`as isize` failed for Note G");
        assert_eq!(Note::Gis as isize, 9, "`as isize` failed for Note Gis");
        assert_eq!(Note::A as isize, 10, "`as isize` failed for Note A");
        assert_eq!(Note::Ais as isize, 11, "`as isize` failed for Note Ais");
        assert_eq!(Note::B as isize, 12, "`as isize` failed for Note B");
    }

    #[test]
    fn transpose_test() {
        let map = vec![
            // +0
            (0, Note::C, Note::C),
            (0, Note::Cis, Note::Cis),
            (0, Note::D, Note::D),
            (0, Note::Dis, Note::Dis),
            (0, Note::E, Note::E),
            (0, Note::F, Note::F),
            (0, Note::Fis, Note::Fis),
            (0, Note::G, Note::G),
            (0, Note::Gis, Note::Gis),
            (0, Note::A, Note::A),
            (0, Note::Ais, Note::Ais),
            (0, Note::B, Note::B),

            // +1
            (1, Note::C, Note::Cis),
            (1, Note::Cis, Note::D),
            (1, Note::D, Note::Dis),
            (1, Note::Dis, Note::E),
            (1, Note::E, Note::F),
            (1, Note::F, Note::Fis),
            (1, Note::Fis, Note::G),
            (1, Note::G, Note::Gis),
            (1, Note::Gis, Note::A),
            (1, Note::A, Note::Ais),
            (1, Note::Ais, Note::B),
            (1, Note::B, Note::C),

            // +12 = +0
            (12, Note::C, Note::C),
            (12, Note::Cis, Note::Cis),
            (12, Note::D, Note::D),
            (12, Note::Dis, Note::Dis),
            (12, Note::E, Note::E),
            (12, Note::F, Note::F),
            (12, Note::Fis, Note::Fis),
            (12, Note::G, Note::G),
            (12, Note::Gis, Note::Gis),
            (12, Note::A, Note::A),
            (12, Note::Ais, Note::Ais),
            (12, Note::B, Note::B),

            // +13 = +1
            (13, Note::C, Note::Cis),
            (13, Note::Cis, Note::D),
            (13, Note::D, Note::Dis),
            (13, Note::Dis, Note::E),
            (13, Note::E, Note::F),
            (13, Note::F, Note::Fis),
            (13, Note::Fis, Note::G),
            (13, Note::G, Note::Gis),
            (13, Note::Gis, Note::A),
            (13, Note::A, Note::Ais),
            (13, Note::Ais, Note::B),
            (13, Note::B, Note::C),

            // +4
            (4, Note::C, Note::E),
            (4, Note::Cis, Note::F),
            (4, Note::D, Note::Fis),
            (4, Note::Dis, Note::G),
            (4, Note::E, Note::Gis),
            (4, Note::F, Note::A),
            (4, Note::Fis, Note::Ais),
            (4, Note::G, Note::B),
            (4, Note::Gis, Note::C),
            (4, Note::A, Note::Cis),
            (4, Note::Ais, Note::D),
            (4, Note::B, Note::Dis),

            // -1
            (-1, Note::C, Note::B),
            (-1, Note::Cis, Note::C),
            (-1, Note::D, Note::Cis),
            (-1, Note::Dis, Note::D),
            (-1, Note::E, Note::Dis),
            (-1, Note::F, Note::E),
            (-1, Note::Fis, Note::F),
            (-1, Note::G, Note::Fis),
            (-1, Note::Gis, Note::G),
            (-1, Note::A, Note::Gis),
            (-1, Note::Ais, Note::A),
            (-1, Note::B, Note::Ais),

            // +11 = -1
            (11, Note::C, Note::B),
            (11, Note::Cis, Note::C),
            (11, Note::D, Note::Cis),
            (11, Note::Dis, Note::D),
            (11, Note::E, Note::Dis),
            (11, Note::F, Note::E),
            (11, Note::Fis, Note::F),
            (11, Note::G, Note::Fis),
            (11, Note::Gis, Note::G),
            (11, Note::A, Note::Gis),
            (11, Note::Ais, Note::A),
            (11, Note::B, Note::Ais),
        ];

        for (semitones, input, expected) in map {
            assert_eq!(input.transpose(semitones), expected, "Transpose failed for {:?} {}", input, semitones);
        }
    }
}
