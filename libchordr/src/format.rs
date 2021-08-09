use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Format {
    HTML,
    Chorddown,
    SongBeamer,
    Text,
    #[cfg(feature = "pdf")]
    PDF,
}

impl Format {
    pub fn get_all() -> &'static [Format] {
        &[
            Self::HTML,
            Self::Chorddown,
            Self::SongBeamer,
            Self::Text,
            #[cfg(feature = "pdf")]
            Self::PDF,
        ]
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::HTML => f.write_str("HTML"),
            Self::Chorddown => f.write_str("Chorddown"),
            Self::SongBeamer => f.write_str("SongBeamer"),
            Self::Text => f.write_str("Text"),
            #[cfg(feature = "pdf")]
            Self::PDF => f.write_str("PDF"),
        }
    }
}

impl TryFrom<&str> for Format {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "html" => Ok(Self::HTML),
            "chorddown" => Ok(Self::Chorddown),
            "songbeamer" => Ok(Self::SongBeamer),
            "text" => Ok(Self::Text),
            #[cfg(feature = "pdf")]
            "pdf" => Ok(Self::PDF),
            _ => Err(()),
        }
    }
}
