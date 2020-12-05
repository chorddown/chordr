mod song_settings_map;

use crate::format::Format;
use crate::models::chord::fmt::Formatting;
use crate::models::meta::{BNotation, SemitoneNotation};
use serde::{Deserialize, Serialize};
pub use song_settings_map::SongSettingsMap;

/// A structure of formatting and transpose settings for a song
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SongSettings {
    transpose_semitone: isize,
    formatting: Formatting,
}

impl SongSettings {
    pub fn new(transpose_semitone: isize, formatting: Formatting) -> Self {
        Self {
            transpose_semitone,
            formatting,
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
}

impl Default for SongSettings {
    fn default() -> Self {
        SongSettings::new(0, Formatting::default())
    }
}
