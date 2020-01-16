mod note;
mod chords;

use crate::error::Error;
use crate::models::meta::BNotation;
pub use self::note::Note;
pub use self::chords::Chords;

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

    pub fn to_string(&self, b_notation: BNotation) -> String {
        match self.variant {
            Some(ref v) => format!("{}{}", self.root.to_string(b_notation), v),
            None => format!("{}", self.root.to_string(b_notation)),
        }
    }

    pub fn try_from(value: &str, b_notation: BNotation) -> Result<Self, Error> {
        if value.is_empty() {
            return Err(Error::chord_error("Given value is empty"));
        }

        if value.len() < 2 {
            let note = Note::try_from(value, b_notation)?;

            return Ok(note.into());
        }

        let (node_raw, variant_raw) = Self::split_note_characters(value)?;

//        println!("ch {:?}", first_non_node_part);
//
        let variant_raw_trimmed = variant_raw.trim();
        let variant = if variant_raw_trimmed.is_empty() {
            None
        } else {
            Some(variant_raw_trimmed.to_owned())
        };

        return Ok(Self {
            root: Note::try_from(&node_raw, b_notation)?,
            variant,
        });
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
            _ => return Err(Error::chord_error(format!("First character '{}' is not a note", chars[0])))
        }
        let note_has_two_chars = match chars[1] {
            '♭' | 'b' | '♯' | '#' => true,
            _ => false
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
}

impl From<Note> for Chord {
    fn from(n: Note) -> Self {
        Self::without_variant(n)
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
}
