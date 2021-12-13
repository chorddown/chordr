use crate::models::chord::Chord;
use crate::models::metadata::{BNotation, Metadata};
use crate::tokenizer::RawMetadata;

#[derive(Clone)]
pub(super) struct MetadataBuilder {
    metadata: Metadata,
    key_raw: Option<String>,
    original_key_raw: Option<String>,
}

/// Additional methods for Metadata used by the Parser to set fields
impl MetadataBuilder {
    pub(super) fn new() -> Self {
        Self {
            metadata: Default::default(),
            key_raw: None,
            original_key_raw: None,
        }
    }

    pub(super) fn build(self) -> Metadata {
        self.metadata
    }

    /// Configure the Builder with the Metadata content copied into the appropriate field
    pub(super) fn assign_from_token(&mut self, token: RawMetadata) {
        match token {
            RawMetadata::Subtitle(c) => self.metadata.subtitle = Some(c),
            RawMetadata::Artist(c) => self.metadata.artist = Some(c),
            RawMetadata::Composer(c) => self.metadata.composer = Some(c),
            RawMetadata::Lyricist(c) => self.metadata.lyricist = Some(c),
            RawMetadata::Copyright(c) => self.metadata.copyright = Some(c),
            RawMetadata::Album(c) => self.metadata.album = Some(c),
            RawMetadata::Year(c) => self.metadata.year = Some(c),
            RawMetadata::Time(c) => self.metadata.time = Some(c),
            RawMetadata::Tempo(c) => self.metadata.tempo = Some(c),
            RawMetadata::Duration(c) => self.metadata.duration = Some(c),
            RawMetadata::Capo(c) => self.metadata.capo = Some(c),
            RawMetadata::OriginalTitle(c) => self.metadata.original_title = Some(c),
            RawMetadata::AlternativeTitle(c) => self.metadata.alternative_title = Some(c),
            RawMetadata::CCLISongId(c) => self.metadata.ccli_song_id = Some(c),
            RawMetadata::Key(c) => self.set_key(c.clone()),
            RawMetadata::OriginalKey(c) => self.set_original_key(c.clone()),
            RawMetadata::BNotation(notation) => self.metadata.b_notation = notation,
        }
    }

    pub(super) fn with_b_notation(mut self, b_notation: BNotation) -> Self {
        self.metadata.b_notation = b_notation;
        self
    }

    pub(super) fn with_title(mut self, title: &str) -> MetadataBuilder {
        self.metadata.title = Some(title.to_string());
        self
    }

    /// Update the `key` and `original_key` fields with a new B-Notation
    pub(crate) fn reinterpret_keys_with_b_notation(&mut self, to_notation: BNotation) {
        if let Some(key) = &self.key_raw {
            self.metadata.key = Chord::try_from(key, to_notation).ok();
        }
        if let Some(key) = &self.original_key_raw {
            self.metadata.original_key = Chord::try_from(key, to_notation).ok();
        }
    }

    fn set_key(&mut self, content: String) {
        self.metadata.key = Chord::try_from(&content, self.metadata.b_notation).ok();
        // self.metadata.key_raw = Some(conten);
        self.key_raw = Some(content);
    }

    fn set_original_key(&mut self, content: String) {
        self.metadata.original_key = Chord::try_from(&content, self.metadata.b_notation).ok();
        // self.metadata.original_key_raw = Some(content.clone());
        self.original_key_raw = Some(content);
    }
}

impl Default for MetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::chord::Chord;
    use crate::models::chord::Note;
    use crate::models::metadata::b_notation::BNotation;
    use crate::parser::metadata_builder::MetadataBuilder;
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
            let mut metadata_builder = MetadataBuilder::default();
            metadata_builder.assign_from_token(RawMetadata::key(case.0));
            assert_eq!(metadata_builder.build().key, Some(case.1));
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
            let mut metadata_builder = MetadataBuilder::default();
            metadata_builder.assign_from_token(RawMetadata::key(case.0));
            metadata_builder.reinterpret_keys_with_b_notation(case.1);
            assert_eq!(metadata_builder.build().key, Some(case.2));
        }
    }
}
