use crate::models::chord::Chord;
use crate::models::metadata::{BNotation, Metadata};
use crate::tokenizer::RawMetadata;

/// Additional methods for Metadata used by the Parser to set fields
impl Metadata {
    /// Copy the Meta content into the appropriate field
    pub(super) fn assign_from_token(&mut self, t: &RawMetadata) {
        match t {
            RawMetadata::Subtitle(content) => self.subtitle = Some(content.clone()),
            RawMetadata::Artist(content) => self.artist = Some(content.clone()),
            RawMetadata::Composer(content) => self.composer = Some(content.clone()),
            RawMetadata::Lyricist(content) => self.lyricist = Some(content.clone()),
            RawMetadata::Copyright(content) => self.copyright = Some(content.clone()),
            RawMetadata::Album(content) => self.album = Some(content.clone()),
            RawMetadata::Year(content) => self.year = Some(content.clone()),
            RawMetadata::Time(content) => self.time = Some(content.clone()),
            RawMetadata::Tempo(content) => self.tempo = Some(content.clone()),
            RawMetadata::Duration(content) => self.duration = Some(content.clone()),
            RawMetadata::Capo(content) => self.capo = Some(content.clone()),
            RawMetadata::OriginalTitle(content) => self.original_title = Some(content.clone()),
            RawMetadata::AlternativeTitle(content) => {
                self.alternative_title = Some(content.clone())
            }
            RawMetadata::CCLISongId(content) => self.ccli_song_id = Some(content.clone()),
            RawMetadata::Key(content) => self.set_key(content.clone()),
            RawMetadata::OriginalKey(content) => self.set_original_key(content.clone()),
            RawMetadata::BNotation(notation) => self.b_notation = *notation,
        }
    }

    /// Update the `key` and `original_key` fields with a new B-Notation
    pub(crate) fn reinterpret_keys_with_b_notation(&mut self, to_notation: BNotation) {
        if let Some(key) = &self.key_raw {
            self.key = Chord::try_from(&key, to_notation).ok();
        }
        if let Some(key) = &self.original_key_raw {
            self.original_key = Chord::try_from(&key, to_notation).ok();
        }
    }

    fn set_key(&mut self, content: String) {
        self.key = Chord::try_from(&content, self.b_notation).ok();
        self.key_raw = Some(content);
    }

    fn set_original_key(&mut self, content: String) {
        self.original_key = Chord::try_from(&content, self.b_notation).ok();
        self.original_key_raw = Some(content);
    }
}

#[cfg(test)]
mod tests {
    use crate::models::chord::Chord;
    use crate::models::chord::Note;
    use crate::models::metadata::b_notation::BNotation;
    use crate::models::metadata::Metadata;
    use crate::tokenizer::RawMetadata;

    #[test]
    fn test_assign_from_token_key() {
        let test_cases = vec![
            ("D", Chord::new_without_variant(Note::D)),
            ("Dm", Chord::new_with_variant(Note::D, "m")),
            ("B", Chord::new_without_variant(Note::B)),
            ("Bb", Chord::new_without_variant(Note::Ais)),
            ("H", Chord::new_without_variant(Note::B)),
        ];

        for case in test_cases {
            let mut metadata = Metadata::default();
            metadata.assign_from_token(&RawMetadata::key(case.0));
            assert_eq!(metadata.key, Some(case.1));
        }
    }
    #[test]
    fn test_update_b_notation() {
        let chord_w = Chord::new_with_variant;
        let chord_wo = Chord::new_without_variant;

        let test_cases = vec![
            // Change to H
            ("D", BNotation::H, chord_wo(Note::D)),
            ("Dm", BNotation::H, chord_w(Note::D, "m")),
            ("A♯", BNotation::H, chord_wo(Note::Ais)),
            ("A#", BNotation::H, chord_wo(Note::Ais)),
            ("B", BNotation::H, chord_wo(Note::Ais)),
            ("B♭", BNotation::H, chord_wo(Note::Ais)),
            ("Bb", BNotation::H, chord_wo(Note::Ais)),
            ("H", BNotation::H, chord_wo(Note::B)),
            // Change to B
            ("D", BNotation::B, chord_wo(Note::D)),
            ("Dm", BNotation::B, chord_w(Note::D, "m")),
            ("A♯", BNotation::B, chord_wo(Note::Ais)),
            ("A#", BNotation::B, chord_wo(Note::Ais)),
            ("B", BNotation::B, chord_wo(Note::B)),
            ("B♭", BNotation::B, chord_wo(Note::Ais)),
            ("Bb", BNotation::B, chord_wo(Note::Ais)),
            ("H", BNotation::B, chord_wo(Note::B)),
        ];

        for case in test_cases {
            let mut metadata = Metadata::default();
            metadata.assign_from_token(&RawMetadata::key(case.0));
            metadata.reinterpret_keys_with_b_notation(case.1);
            assert_eq!(metadata.key, Some(case.2));
        }
    }
}
