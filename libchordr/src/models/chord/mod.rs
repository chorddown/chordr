mod chords;
pub mod fmt;
mod note;
mod transposition;

pub use self::chords::Chords;
pub use self::fmt::NoteDisplay;
pub use self::note::Note;
pub use self::transposition::TransposableTrait;
use crate::error::Error;
use crate::models::chord::fmt::Formatting;
use crate::models::meta::BNotation;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Chord {
    root: Note,
    variant: Option<String>,
}

impl Chord {
    pub fn with_variant<S: Into<String>>(root: Note, variant: S) -> Self {
        Self {
            root,
            variant: Some(variant.into()),
        }
    }

    pub fn without_variant(root: Note) -> Self {
        Self {
            root,
            variant: None,
        }
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
        let note_has_two_chars = match chars[1] {
            '♭' | 'b' | '♯' | '#' => true,
            _ => false,
        };

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
        Self::without_variant(n)
    }
}

impl TransposableTrait for Chord {
    fn transpose(&self, semitones: isize) -> Self {
        Self {
            root: self.root.transpose(semitones),
            variant: self.variant.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_without_variant() {
        let chord = Chord::try_from("A", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::A);
        assert_eq!(chord.variant, None);
        let chord = Chord::try_from("A#", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, None);

        let chord = Chord::try_from("C#", BNotation::H).unwrap();
        assert_eq!(chord.root, Note::Cis);
        assert_eq!(chord.variant, None);

        let chord = Chord::try_from("Bb", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, None);
        let chord = Chord::try_from("bb", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, None);

        let chord = Chord::try_from("B", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::B);
        assert_eq!(chord.variant, None);
        let chord = Chord::try_from("B", BNotation::H).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, None);
        let chord = Chord::try_from("B", BNotation::H).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, None);
        assert!(Chord::try_from("H#", BNotation::H).is_err());

        let chord = Chord::try_from("F", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::F);
        assert_eq!(chord.variant, None);
        let chord = Chord::try_from("F#", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::Fis);
        assert_eq!(chord.variant, None);
        assert!(Chord::try_from("Fb", BNotation::H).is_err());
    }

    #[test]
    fn try_from_with_variant() {
        let chord = Chord::try_from("Am", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::A);
        assert_eq!(chord.variant, Some("m".to_owned()));
        let chord = Chord::try_from("A♯dim", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, Some("dim".to_owned()));

        let chord = Chord::try_from("C#madd2add4", BNotation::H).unwrap();
        assert_eq!(chord.root, Note::Cis);
        assert_eq!(chord.variant, Some("madd2add4".to_owned()));

        let chord = Chord::try_from("Bbmaj7", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, Some("maj7".to_owned()));
        let chord = Chord::try_from("bbm", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, Some("m".to_owned()));

        let chord = Chord::try_from("B7", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::B);
        assert_eq!(chord.variant, Some("7".to_owned()));
        let chord = Chord::try_from("B9", BNotation::H).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, Some("9".to_owned()));
        let chord = Chord::try_from("Bm#5", BNotation::H).unwrap();
        assert_eq!(chord.root, Note::Ais);
        assert_eq!(chord.variant, Some("m#5".to_owned()));
        assert!(Chord::try_from("H#", BNotation::H).is_err());

        let chord = Chord::try_from("Fmaj13#11", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::F);
        assert_eq!(chord.variant, Some("maj13#11".to_owned()));
        let chord = Chord::try_from("F#7sus4", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::Fis);
        assert_eq!(chord.variant, Some("7sus4".to_owned()));
        assert!(Chord::try_from("Fb", BNotation::H).is_err());

        let chord = Chord::try_from("D#9Daniel", BNotation::B).unwrap();
        assert_eq!(chord.root, Note::Dis);
        assert_eq!(chord.variant, Some("9Daniel".to_owned()));
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
