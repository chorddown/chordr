use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Format {
    HTML,
    Chorddown,
    SongBeamer,
    #[cfg(feature = "pdf")]
    PDF,
}

impl Format {
    pub fn get_all() -> &'static [Format] {
        &[
            Self::HTML,
            Self::Chorddown,
            Self::SongBeamer,
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
            #[cfg(feature = "pdf")]
            "pdf" => Ok(Self::PDF),
            _ => Err(()),
        }
    }
}
