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
    pub fn formatting(&self) -> Formatting {
        self.formatting
    }
}
