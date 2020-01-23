use crate::models::meta::{BNotation, SemitoneNotation};
use crate::format::Format;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Formatting {
    pub b_notation: BNotation,
    pub semitone_notation: SemitoneNotation,
    pub format: Format,
}

impl Formatting {
    pub fn with_format(format: Format) -> Self {
        Self {
            b_notation: BNotation::default(),
            semitone_notation: Default::default(),
            format,
        }
    }
}

impl Default for Formatting {
    fn default() -> Self {
        Self {
            b_notation: BNotation::default(),
            semitone_notation: SemitoneNotation::default(),
            format: Format::HTML,
        }
    }
}

pub trait NoteDisplay {
    fn to_string(&self, format: Formatting) -> String;
}
