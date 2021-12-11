use crate::models::chord::{Chord, TransposableTrait};
use crate::models::metadata::b_notation::BNotation;
use crate::models::metadata::MetadataTrait;
use crate::tokenizer::Meta;

/// Metadata of a parsed song
#[derive(Clone, Debug)]
pub struct Metadata {
    pub(crate) title: Option<String>,
    pub(crate) subtitle: Option<String>,
    pub(crate) artist: Option<String>,
    pub(crate) composer: Option<String>,
    pub(crate) lyricist: Option<String>,
    pub(crate) copyright: Option<String>,
    pub(crate) album: Option<String>,
    pub(crate) year: Option<String>,
    pub(crate) key: Option<Chord>,
    pub(crate) key_raw: Option<String>,
    pub(crate) original_key: Option<Chord>,
    pub(crate) original_key_raw: Option<String>,
    pub(crate) time: Option<String>,
    pub(crate) tempo: Option<String>,
    pub(crate) duration: Option<String>,
    pub(crate) capo: Option<String>,
    pub(crate) original_title: Option<String>,
    pub(crate) alternative_title: Option<String>,
    pub(crate) ccli_song_id: Option<String>,
    pub(crate) b_notation: BNotation,
}

impl Metadata {
    /// Copy the Meta content into the appropriate field
    pub(crate) fn assign_from_token(&mut self, t: &Meta) {
        match t {
            Meta::Subtitle(content) => self.subtitle = Some(content.clone()),
            Meta::Artist(content) => self.artist = Some(content.clone()),
            Meta::Composer(content) => self.composer = Some(content.clone()),
            Meta::Lyricist(content) => self.lyricist = Some(content.clone()),
            Meta::Copyright(content) => self.copyright = Some(content.clone()),
            Meta::Album(content) => self.album = Some(content.clone()),
            Meta::Year(content) => self.year = Some(content.clone()),
            Meta::Time(content) => self.time = Some(content.clone()),
            Meta::Tempo(content) => self.tempo = Some(content.clone()),
            Meta::Duration(content) => self.duration = Some(content.clone()),
            Meta::Capo(content) => self.capo = Some(content.clone()),
            Meta::OriginalTitle(content) => self.original_title = Some(content.clone()),
            Meta::AlternativeTitle(content) => self.alternative_title = Some(content.clone()),
            Meta::CCLISongId(content) => self.ccli_song_id = Some(content.clone()),
            Meta::Key(content) => self.set_key(content.clone()),
            Meta::OriginalKey(content) => self.set_original_key(content.clone()),
            Meta::BNotation(notation) => self.b_notation = *notation,
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

impl MetadataTrait for Metadata {
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }

    fn artist(&self) -> Option<&str> {
        self.artist.as_deref()
    }

    fn composer(&self) -> Option<&str> {
        self.composer.as_deref()
    }

    fn lyricist(&self) -> Option<&str> {
        self.lyricist.as_deref()
    }

    fn copyright(&self) -> Option<&str> {
        self.copyright.as_deref()
    }

    fn album(&self) -> Option<&str> {
        self.album.as_deref()
    }

    fn year(&self) -> Option<&str> {
        self.year.as_deref()
    }

    fn key(&self) -> Option<&Chord> {
        self.key.as_ref()
    }

    fn original_key(&self) -> Option<&Chord> {
        self.original_key.as_ref()
    }

    fn time(&self) -> Option<&str> {
        self.time.as_deref()
    }

    fn tempo(&self) -> Option<&str> {
        self.tempo.as_deref()
    }

    fn duration(&self) -> Option<&str> {
        self.duration.as_deref()
    }

    fn capo(&self) -> Option<&str> {
        self.capo.as_deref()
    }

    fn original_title(&self) -> Option<&str> {
        self.original_title.as_deref()
    }

    fn alternative_title(&self) -> Option<&str> {
        self.alternative_title.as_deref()
    }

    fn ccli_song_id(&self) -> Option<&str> {
        self.ccli_song_id.as_deref()
    }

    fn b_notation(&self) -> BNotation {
        self.b_notation
    }
}

impl TransposableTrait for Metadata {
    fn transpose(self, semitones: isize) -> Self {
        if self.key_raw.is_some() && self.original_key_raw.is_none() {
            let mut transposed_meta = self.clone();
            let key = transposed_meta.key.take();
            if transposed_meta.original_key.is_none() {
                transposed_meta.original_key = key.clone();
            }
            transposed_meta.key = Some(key.unwrap().transpose(semitones));

            transposed_meta
        } else {
            self
        }
    }
}
impl Default for Metadata {
    fn default() -> Self {
        Self {
            title: None,
            subtitle: None,
            artist: None,
            composer: None,
            lyricist: None,
            copyright: None,
            album: None,
            year: None,
            key: None,
            key_raw: None,
            original_key: None,
            original_key_raw: None,
            time: None,
            tempo: None,
            duration: None,
            capo: None,
            original_title: None,
            alternative_title: None,
            ccli_song_id: None,
            b_notation: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::chord::Note;

    use super::*;

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
            let mut meta = Metadata::default();
            meta.assign_from_token(&Meta::key(case.0));
            assert_eq!(meta.key, Some(case.1));
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
            let mut meta = Metadata::default();
            meta.assign_from_token(&Meta::key(case.0));
            meta.reinterpret_keys_with_b_notation(case.1);
            assert_eq!(meta.key, Some(case.2));
        }
    }
}
