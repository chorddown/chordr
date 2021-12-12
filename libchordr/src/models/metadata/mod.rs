use serde::{Deserialize, Serialize};

pub use crate::metadata::metadata_trait::MetadataTrait;
use crate::models::chord::{Chord, NoteDisplay, TransposableTrait};
use crate::prelude::Formatting;

pub use self::b_notation::BNotation;
pub use self::semitone_notation::SemitoneNotation;

pub mod b_notation;
pub mod semitone_notation;

/// Metadata of a parsed song
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
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
            let mut transposed_meta = self;
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

impl From<&dyn MetadataTrait> for Metadata {
    fn from(input: &dyn MetadataTrait) -> Self {
        fn option_to_owned(option: Option<&str>) -> Option<String> {
            option.map(ToOwned::to_owned)
        }
        fn chord_option_to_string(option: Option<&Chord>, b_notation: BNotation) -> Option<String> {
            option.map(|k| k.note_format(Formatting::default().with_b_notation(b_notation)))
        }
        Metadata {
            title: option_to_owned(input.title()),
            subtitle: option_to_owned(input.subtitle()),
            artist: option_to_owned(input.artist()),
            composer: option_to_owned(input.composer()),
            lyricist: option_to_owned(input.lyricist()),
            copyright: option_to_owned(input.copyright()),
            album: option_to_owned(input.album()),
            year: option_to_owned(input.year()),
            key: input.key().map(ToOwned::to_owned),
            key_raw: chord_option_to_string(input.key(), input.b_notation()),
            original_key: input.original_key().map(ToOwned::to_owned),
            original_key_raw: chord_option_to_string(input.original_key(), input.b_notation()),
            time: option_to_owned(input.time()),
            tempo: option_to_owned(input.tempo()),
            duration: option_to_owned(input.duration()),
            capo: option_to_owned(input.capo()),
            original_title: option_to_owned(input.original_title()),
            alternative_title: option_to_owned(input.alternative_title()),
            ccli_song_id: option_to_owned(input.ccli_song_id()),
            b_notation: input.b_notation(),
        }
    }
}
