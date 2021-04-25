use serde::{Deserialize, Serialize};

pub use song_settings_map::SongSettingsMap;

use crate::format::Format;
use crate::models::chord::fmt::Formatting;
use crate::models::meta::{BNotation, SemitoneNotation};

mod song_settings_map;

/// A structure of formatting and transpose settings for a song
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SongSettings {
    transpose_semitone: isize,
    formatting: Formatting,
    #[serde(default)]
    note: String,
}

impl SongSettings {
    pub fn new<S: Into<String>>(
        transpose_semitone: isize,
        formatting: Formatting,
        note: S,
    ) -> Self {
        Self {
            transpose_semitone,
            formatting,
            note: note.into(),
        }
    }
    pub fn b_notation(&self) -> BNotation {
        self.formatting.b_notation
    }
    pub fn semitone_notation(&self) -> SemitoneNotation {
        self.formatting.semitone_notation
    }
    pub fn format(&self) -> Format {
        self.formatting.format
    }
    pub fn transpose_semitone(&self) -> isize {
        self.transpose_semitone
    }
    pub fn with_transpose_semitone(&self, transpose_semitone: isize) -> Self {
        let mut clone = self.clone();
        clone.transpose_semitone = transpose_semitone;

        clone
    }

    pub fn formatting(&self) -> Formatting {
        self.formatting
    }
    pub fn with_formatting(&self, formatting: Formatting) -> Self {
        let mut clone = self.clone();
        clone.formatting = formatting;

        clone
    }

    pub fn note(&self) -> &str {
        self.note.as_ref()
    }

    pub fn with_note(&self, note: String) -> Self {
        let mut clone = self.clone();
        clone.note = note;

        clone
    }
}

impl Default for SongSettings {
    fn default() -> Self {
        SongSettings::new(0, Formatting::default(), String::new())
    }
}
