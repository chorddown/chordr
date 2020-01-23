use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Format {
    HTML,
    Chorddown,
    SongBeamer,
    #[cfg(feature = "pdf")]
    PDF,
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
            _ => Err(())
        }
    }
}
