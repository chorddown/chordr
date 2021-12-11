use serde::de::{self, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Deserializer, Serialize};

use crate::error::Error;
use crate::format::Format;
use crate::models::chord::fmt::Formatting;
use crate::models::metadata::BNotation;
use crate::prelude::SemitoneNotation;

pub use self::chords::Chords;
pub use self::fmt::NoteDisplay;
pub use self::note::Note;
pub use self::transposition::TransposableTrait;

mod chords;
pub mod fmt;
mod note;
mod transposition;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Chord {
    root: Note,
    variant: Option<String>,
}

impl Chord {
    #[deprecated(note = "Use new_with_variant")]
    pub fn with_variant<S: Into<String>>(root: Note, variant: S) -> Self {
        Self::new_with_variant(root, variant.into())
    }

    pub fn new_with_variant<S: Into<String>>(root: Note, variant: S) -> Self {
        Self {
            root,
            variant: Some(variant.into()),
        }
    }

    #[deprecated(note = "Use new_without_variant")]
    pub fn without_variant(root: Note) -> Self {
        Self::new_without_variant(root)
    }

    pub fn new_without_variant(root: Note) -> Self {
        Self {
            root,
            variant: None,
        }
    }

    pub fn root(&self) -> Note {
        self.root
    }

    pub fn with_root(self, root: Note) -> Self {
        Self {
            root,
            variant: self.variant,
        }
    }

    pub fn variant(&self) -> Option<&str> {
        self.variant.as_ref().map(String::as_str)
    }

    pub fn try_from(value: &str, b_notation: BNotation) -> Result<Self, Error> {
        if value.is_empty() {
            return Err(Error::chord_error("Given chord brackets are empty"));
        }

        if value.len() < 2 {
            let note = Note::try_from(value, b_notation)?;

            return Ok(note.into());
        }

        let (node_raw, variant_raw) = Self::split_note_characters(value)?;
        let variant_raw_trimmed = variant_raw.trim();
        let variant = if variant_raw_trimmed.is_empty() {
            None
        } else {
            Some(variant_raw_trimmed.to_owned())
        };

        Ok(Self {
            root: Note::try_from(&node_raw, b_notation)?,
            variant,
        })
    }

    fn split_note_characters(value: &str) -> Result<(String, String), Error> {
        if value.is_empty() {
            return Err(Error::chord_error("Given root is empty"));
        }

        assert!(!value.is_empty(), "Value must not be empty");
        assert!(value.len() >= 2);
        let chars: Vec<char> = value.chars().collect();

        match chars[0] {
            'A' | 'H' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' => (),
            'a' | 'h' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' => (),
            _ => {
                return Err(Error::chord_error(format!(
                    "First character '{}' is not a note",
                    chars[0]
                )))
            }
        }
        let note_has_two_chars = matches!(chars[1], '♭' | 'b' | '♯' | '#');

        let (node_raw, variant_raw) = if note_has_two_chars {
            chars.split_at(2)
        } else {
            chars.split_at(1)
        };

        Ok((
            node_raw.iter().collect::<String>(),
            variant_raw.iter().collect::<String>(),
        ))
    }

    //    fn to_string(&self, b_notation: BNotation, sn: SemitoneNotation) -> &str {
    //        match self.variant {
    //            Some(ref v) => format!("{}{}", NoteDisplay::to_string(&self.root, b_notation, sn), v).as_str(),
    //            None => format!("{}", NoteDisplay::to_string(&self.root, b_notation, sn)).as_str(),
    //        }
    //    }
}

impl From<Note> for Chord {
    fn from(n: Note) -> Self {
        Self::new_without_variant(n)
    }
}

impl TransposableTrait for Chord {
    fn transpose(self, semitones: isize) -> Self {
        Self {
            root: self.root.transpose(semitones),
            variant: self.variant,
        }
    }
}

impl NoteDisplay for Chord {
    fn note_format(&self, formatting: Formatting) -> String {
        match self.variant {
            Some(ref v) => format!("{}{}", NoteDisplay::note_format(&self.root, formatting), v),
            None => NoteDisplay::note_format(&self.root, formatting),
        }
    }
}

fn get_serialization_b_notation() -> BNotation {
    BNotation::B
}
fn get_serialization_formatting() -> Formatting {
    Formatting {
        b_notation: get_serialization_b_notation(),
        semitone_notation: SemitoneNotation::Sharp,
        format: Format::HTML,
    }
}

impl Serialize for Chord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.note_format(get_serialization_formatting()))
    }
}

struct ChordVisitor;

impl<'de> Visitor<'de> for ChordVisitor {
    type Value = Chord;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a formatted chord")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Chord::try_from(v, get_serialization_b_notation()) {
            Ok(c) => Ok(c),
            Err(e) => Err(E::custom(e.to_string())),
        }
    }
}
impl<'de> Deserialize<'de> for Chord {
    fn deserialize<D>(deserializer: D) -> Result<Chord, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ChordVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn without_variant_cases() -> Vec<(&'static str, BNotation, Note)> {
        vec![
            ("A", BNotation::B, Note::A),
            ("A#", BNotation::B, Note::Ais),
            ("C#", BNotation::H, Note::Cis),
            ("Bb", BNotation::B, Note::Ais),
            ("bb", BNotation::B, Note::Ais),
            ("B", BNotation::B, Note::B),
            ("B", BNotation::H, Note::Ais),
            ("F", BNotation::B, Note::F),
            ("F#", BNotation::B, Note::Fis),
        ]
    }
    fn with_variant_cases() -> Vec<(&'static str, BNotation, Note, &'static str)> {
        vec![
            ("Am", BNotation::B, Note::A, "m"),
            ("A♯dim", BNotation::B, Note::Ais, "dim"),
            ("C#madd2add4", BNotation::B, Note::Cis, "madd2add4"),
            ("Bbmaj7", BNotation::B, Note::Ais, "maj7"),
            ("bbm", BNotation::B, Note::Ais, "m"),
            ("B7", BNotation::B, Note::B, "7"),
            ("B9", BNotation::H, Note::Ais, "9"),
            ("Bm#5", BNotation::H, Note::Ais, "m#5"),
            ("Fmaj13#11", BNotation::B, Note::F, "maj13#11"),
            ("F#7sus4", BNotation::B, Note::Fis, "7sus4"),
            ("D#9Daniel", BNotation::B, Note::Dis, "9Daniel"),
        ]
    }
    #[test]
    fn try_from_without_variant() {
        for case in without_variant_cases() {
            let chord = Chord::try_from(case.0, case.1).unwrap();
            assert_eq!(chord.root, case.2);
            assert_eq!(chord.variant, None);
        }
        assert!(Chord::try_from("H#", BNotation::H).is_err());
        assert!(Chord::try_from("Fb", BNotation::H).is_err());
    }

    #[test]
    fn try_from_with_variant() {
        for case in with_variant_cases() {
            let chord = Chord::try_from(case.0, case.1).unwrap();
            assert_eq!(chord.root, case.2);
            assert_eq!(chord.variant, Some(case.3.to_string()));
        }
        assert!(Chord::try_from("H#", BNotation::H).is_err());
        assert!(Chord::try_from("Fb", BNotation::H).is_err());
    }

    #[test]
    fn serialize() {
        let test_cases = vec![
            ("A", "A"),
            ("A#", "A#"),
            ("C#", "C#"),
            ("Bb", "A#"),
            ("bb", "A#"),
            ("B", "B"),
            ("F", "F"),
            ("F#", "F#"),
            ("Am", "Am"),
            ("A♯dim", "A#dim"),
            ("C#madd2add4", "C#madd2add4"),
            ("Bbmaj7", "A#maj7"),
            ("bbm", "A#m"),
            ("B7", "B7"),
            ("B9", "B9"),
            ("Bm#5", "Bm#5"),
            ("Fmaj13#11", "Fmaj13#11"),
            ("F#7sus4", "F#7sus4"),
            ("D#9Daniel", "D#9Daniel"),
        ];
        for case in test_cases {
            let chord = Chord::try_from(case.0, BNotation::B).unwrap();
            assert_eq!(
                format!("\"{}\"", case.1),
                serde_json::to_string(&chord).unwrap()
            );
        }
    }

    #[test]
    fn deserialize_without_variant_test() {
        let test_cases = vec![
            ("A", Note::A),
            ("A#", Note::Ais),
            ("Bb", Note::Ais),
            ("bb", Note::Ais),
            ("B", Note::B),
            ("F", Note::F),
            ("F#", Note::Fis),
        ];
        for case in test_cases {
            let failure_msg = format!("Test case for input {} failed", case.0);
            let chord: Chord =
                serde_json::from_str(&format!("\"{}\"", case.0)).expect(&failure_msg);
            assert_eq!(chord.root, case.1, "{}", failure_msg);
            assert_eq!(chord.variant, None, "{}", failure_msg);
        }
    }

    #[test]
    fn deserialize_with_variant_test() {
        let test_cases = vec![
            ("Am", Note::A, "m"),
            ("A♯dim", Note::Ais, "dim"),
            ("C#madd2add4", Note::Cis, "madd2add4"),
            ("Bbmaj7", Note::Ais, "maj7"),
            ("bbm", Note::Ais, "m"),
            ("B7", Note::B, "7"),
            ("Fmaj13#11", Note::F, "maj13#11"),
            ("F#7sus4", Note::Fis, "7sus4"),
            ("D#9Daniel", Note::Dis, "9Daniel"),
        ];

        for case in test_cases {
            let failure_msg = format!("Test case for input {} failed", case.0);
            let chord: Chord =
                serde_json::from_str(&format!("\"{}\"", case.0)).expect(&failure_msg);
            assert_eq!(chord.root, case.1, "{}", failure_msg);
            assert_eq!(chord.variant, Some(case.2.to_string()), "{}", failure_msg);
        }
    }

    #[test]
    fn transpose_test() {
        let map = vec![
            // +0
            (0, "C", Note::C),
            (0, "C#", Note::Cis),
            (0, "D", Note::D),
            (0, "D#", Note::Dis),
            (0, "E", Note::E),
            (0, "F", Note::F),
            (0, "B", Note::B),
            // +1
            (1, "C", Note::Cis),
            (1, "C#", Note::D),
            (1, "D", Note::Dis),
            (1, "D#", Note::E),
            (1, "E", Note::F),
            (1, "F", Note::Fis),
            (1, "A#", Note::B),
            (1, "B", Note::C),
            // +12 = +0
            (12, "C", Note::C),
            (12, "C#", Note::Cis),
            (12, "D", Note::D),
            (12, "G#", Note::Gis),
            (12, "A", Note::A),
            (12, "A#", Note::Ais),
            (12, "B", Note::B),
            // +13 = +1
            (13, "C", Note::Cis),
            (13, "C#", Note::D),
            (13, "G#", Note::A),
            (13, "A", Note::Ais),
            (13, "A#", Note::B),
            (13, "B", Note::C),
            // +4
            (4, "C", Note::E),
            (4, "C#", Note::F),
            (4, "E", Note::Gis),
            (4, "F", Note::A),
            (4, "F#", Note::Ais),
            (4, "G", Note::B),
            (4, "B", Note::Dis),
            // -1
            (-1, "C", Note::B),
            (-1, "C#", Note::C),
            (-1, "E", Note::Dis),
            (-1, "F", Note::E),
            (-1, "F#", Note::F),
            (-1, "B", Note::Ais),
            // +11 = -1
            (11, "C", Note::B),
            (11, "C#", Note::C),
            (11, "D", Note::Cis),
            (11, "E", Note::Dis),
            (11, "F", Note::E),
            (11, "F#", Note::F),
            (11, "G", Note::Fis),
            (11, "B", Note::Ais),
        ];

        for (semitones, input, expected_root) in &map {
            let chord =
                Chord::try_from(input, BNotation::B).expect(&format!("Bad test data {}", input));
            let transposed = chord.transpose(*semitones);
            assert_eq!(
                transposed.root, *expected_root,
                "Transpose failed for {:?} {}",
                input, semitones
            );
            assert_eq!(
                transposed.variant, None,
                "Transpose changed the variant for {:?} {}",
                input, semitones
            );
        }
        for (semitones, input, expected_root) in &map {
            let chord = Chord::try_from(&format!("{}dim", input), BNotation::B)
                .expect(&format!("Bad test data {}", input));
            let transposed = chord.transpose(*semitones);
            assert_eq!(
                transposed.root, *expected_root,
                "Transpose failed for {:?} {}",
                input, semitones
            );
            assert_eq!(
                transposed.variant,
                Some("dim".to_owned()),
                "Transpose changed the variant for {:?} {}",
                input,
                semitones
            );
        }
    }
}
