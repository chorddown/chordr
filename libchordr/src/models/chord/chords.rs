use crate::models::chord::Chord;
use crate::models::meta::BNotation;
use crate::error::Error;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Chords(Chord, Option<Chord>);

impl Chords {
    pub fn try_from(value: &str, b_notation: BNotation) -> Result<Self, Error> {
        if value.is_empty() {
            return Err(Error::chord_error("Given value is empty"));
        }

        let mut inner: Vec<Chord> = Vec::with_capacity(2);
        for p in value.splitn(2, '/') {
            let trimmed_input = p.trim();
            if !trimmed_input.is_empty() {
                inner.push(Chord::try_from(p.trim(), b_notation)?);
            }
        }

        let chord1 = inner.remove(0);
        let chord2 = if inner.len() > 0 {
            Some(inner.remove(0))
        } else {
            None
        };

        Ok(Chords(chord1, chord2))
    }

    pub fn to_string(&self, b_notation: BNotation) -> String {
        match &self.1 {
            None => self.0.to_string(b_notation),
            Some(c) => format!("{}/{}", self.0.to_string(b_notation), c.to_string(b_notation)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::chord::Note;

    #[test]
    fn try_from() {
        let chord_result = Chords::try_from("A /", BNotation::B);
        assert!(chord_result.is_ok(), "{}", chord_result.unwrap_err());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::A);
        assert_eq!(chord.0.variant, None);
        let chord_result = Chords::try_from("A#", BNotation::B);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Ais);
        assert_eq!(chord.0.variant, None);

        let chord_result = Chords::try_from("C#madd2add4/D", BNotation::H);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Cis);
        assert_eq!(chord.0.variant, Some("madd2add4".to_owned()));
        let chord1 = chord.1.unwrap();
        assert_eq!(chord1.root, Note::D);
        assert_eq!(chord1.variant, None);

        let chord_result = Chords::try_from("C#madd2add4/B", BNotation::H);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Cis);
        assert_eq!(chord.0.variant, Some("madd2add4".to_owned()));
        let chord1 = chord.1.unwrap();
        assert_eq!(chord1.root, Note::Ais);
        assert_eq!(chord1.variant, None);

        let chord_result = Chords::try_from("Bbmaj7/C#", BNotation::B);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Ais);
        assert_eq!(chord.0.variant, Some("maj7".to_owned()));
        let chord1 = chord.1.unwrap();
        assert_eq!(chord1.root, Note::Cis);
        assert_eq!(chord1.variant, None);

        let chord_result = Chords::try_from("Bbmaj7/D#m", BNotation::B);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Ais);
        assert_eq!(chord.0.variant, Some("maj7".to_owned()));
        let chord1 = chord.1.unwrap();
        assert_eq!(chord1.root, Note::Dis);
        assert_eq!(chord1.variant, Some("m".to_owned()));

        let chord_result = Chords::try_from("C#", BNotation::H);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Cis);
        assert_eq!(chord.0.variant, None);

        let chord_result = Chords::try_from("Bb", BNotation::B);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Ais);
        assert_eq!(chord.0.variant, None);
        let chord_result = Chords::try_from("bb", BNotation::B);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Ais);
        assert_eq!(chord.0.variant, None);

        let chord_result = Chords::try_from("B", BNotation::B);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::B);
        assert_eq!(chord.0.variant, None);
        let chord_result = Chords::try_from("B", BNotation::H);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Ais);
        assert_eq!(chord.0.variant, None);
        let chord_result = Chords::try_from("B", BNotation::H);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Ais);
        assert_eq!(chord.0.variant, None);
        assert!(Chords::try_from("H#", BNotation::H).is_err());

        let chord_result = Chords::try_from("F", BNotation::B);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::F);
        assert_eq!(chord.0.variant, None);
        let chord_result = Chords::try_from("F#", BNotation::B);
        assert!(chord_result.is_ok());
        let chord = chord_result.unwrap();
        assert_eq!(chord.0.root, Note::Fis);
        assert_eq!(chord.0.variant, None);
        assert!(Chords::try_from("Fb", BNotation::H).is_err());
    }
}
