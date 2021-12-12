use crate::format::Format;
use crate::models::metadata::{BNotation, SemitoneNotation};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Formatting {
    pub b_notation: BNotation,
    pub semitone_notation: SemitoneNotation,
    pub format: Format,
}

impl Formatting {
    #[deprecated(note = "Please use new_with_format()")]
    pub fn with_format(format: Format) -> Self {
        Self {
            b_notation: BNotation::default(),
            semitone_notation: Default::default(),
            format,
        }
    }

    pub fn new_with_format(format: Format) -> Self {
        Self {
            b_notation: BNotation::default(),
            semitone_notation: Default::default(),
            format,
        }
    }

    pub fn with_b_notation(self, b_notation: BNotation) -> Self {
        Self { b_notation, ..self }
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
    fn note_format(&self, format: Formatting) -> String;
}
