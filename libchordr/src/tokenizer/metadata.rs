use std::convert::TryFrom;

use crate::metadata::keyword;
use crate::models::metadata::BNotation;

/// Metadata gathered during tokenization
///
/// Some, but not all metadata can be retrieved during tokenization. The song title e.g.
/// will be determined by the Parser
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum RawMetadata {
    Subtitle(String),
    Artist(String),
    Composer(String),
    Lyricist(String),
    Copyright(String),
    Album(String),
    Year(String),
    Key(String),
    OriginalKey(String),
    Time(String),
    Tempo(String),
    Duration(String),
    Capo(String),
    OriginalTitle(String),
    AlternativeTitle(String),
    CCLISongId(String),
    BNotation(BNotation),
}

impl RawMetadata {
    fn from_keyword_and_content(word: &str, content: &str) -> Option<Self> {
        let content = content.trim();
        match word.trim().to_lowercase().as_str() {
            keyword::ARTIST => Some(Self::artist(content)),
            keyword::COMPOSER => Some(Self::composer(content)),
            keyword::LYRICIST => Some(Self::lyricist(content)),
            keyword::COPYRIGHT => Some(Self::copyright(content)),
            keyword::ALBUM => Some(Self::album(content)),
            keyword::YEAR => Some(Self::year(content)),
            keyword::KEY => Some(Self::key(content)),
            keyword::ORIGINAL_KEY | "original_key" | "original key" | "originalkey" => {
                Some(Self::original_key(content))
            }
            keyword::TIME => Some(Self::time(content)),
            keyword::TEMPO => Some(Self::tempo(content)),
            keyword::DURATION => Some(Self::duration(content)),
            keyword::SUBTITLE => Some(Self::subtitle(content)),
            keyword::CAPO => Some(Self::capo(content)),
            keyword::ORIGINAL_TITLE | "original_title" | "original title" | "originaltitle" => {
                Some(Self::original_title(content))
            }
            keyword::ALTERNATIVE_TITLE | "alternative_title" | "alternative title" => {
                Some(Self::alternative_title(content))
            }
            keyword::CCLI_SONG_ID
            | "ccli song #"
            | "ccli song"
            | "ccli song id"
            | "ccli_song_id" => Some(Self::ccli_song_id(content)),
            keyword::B_NOTATION | "bnotation" | "b notation" | "b_notation" => {
                Some(Self::b_notation(content))
            }
            _ => None,
        }
    }

    /// Return a descriptive title for the metadata
    pub fn label(&self) -> &'static str {
        match self {
            Self::Artist(_) => "Artist",
            Self::Composer(_) => "Composer",
            Self::Lyricist(_) => "Lyricist",
            Self::Copyright(_) => "Copyright",
            Self::Album(_) => "Album",
            Self::Year(_) => "Year",
            Self::Key(_) => "Key",
            Self::OriginalKey(_) => "Original Key",
            Self::Time(_) => "Time",
            Self::Tempo(_) => "Tempo",
            Self::Duration(_) => "Duration",
            Self::Subtitle(_) => "Subtitle",
            Self::Capo(_) => "Capo",
            Self::OriginalTitle(_) => "Original Title",
            Self::AlternativeTitle(_) => "Alternative Title",
            Self::CCLISongId(_) => "CCLI Song #",
            Self::BNotation(_) => "B-Notation",
        }
    }

    pub fn content(&self) -> &str {
        match self {
            Self::Artist(c) => c,
            Self::Composer(c) => c,
            Self::Lyricist(c) => c,
            Self::Copyright(c) => c,
            Self::Album(c) => c,
            Self::Year(c) => c,
            Self::Key(c) => c,
            Self::OriginalKey(c) => c,
            Self::Time(c) => c,
            Self::Tempo(c) => c,
            Self::Duration(c) => c,
            Self::Subtitle(c) => c,
            Self::Capo(c) => c,
            Self::OriginalTitle(c) => c,
            Self::AlternativeTitle(c) => c,
            Self::CCLISongId(c) => c,
            Self::BNotation(c) => c.as_str(),
        }
    }

    pub fn artist<S: Into<String>>(content: S) -> Self {
        Self::Artist(content.into())
    }

    pub fn composer<S: Into<String>>(content: S) -> Self {
        Self::Composer(content.into())
    }

    pub fn lyricist<S: Into<String>>(content: S) -> Self {
        Self::Lyricist(content.into())
    }

    pub fn copyright<S: Into<String>>(content: S) -> Self {
        Self::Copyright(content.into())
    }

    pub fn album<S: Into<String>>(content: S) -> Self {
        Self::Album(content.into())
    }

    pub fn year<S: Into<String>>(content: S) -> Self {
        Self::Year(content.into())
    }

    pub fn key<S: Into<String>>(content: S) -> Self {
        Self::Key(content.into())
    }
    pub fn original_key<S: Into<String>>(content: S) -> Self {
        Self::OriginalKey(content.into())
    }

    pub fn time<S: Into<String>>(content: S) -> Self {
        Self::Time(content.into())
    }

    pub fn tempo<S: Into<String>>(content: S) -> Self {
        Self::Tempo(content.into())
    }

    pub fn duration<S: Into<String>>(content: S) -> Self {
        Self::Duration(content.into())
    }

    pub fn subtitle<S: Into<String>>(content: S) -> Self {
        Self::Subtitle(content.into())
    }

    pub fn capo<S: Into<String>>(content: S) -> Self {
        Self::Capo(content.into())
    }

    pub fn original_title<S: Into<String>>(content: S) -> Self {
        Self::OriginalTitle(content.into())
    }

    pub fn alternative_title<S: Into<String>>(content: S) -> Self {
        Self::AlternativeTitle(content.into())
    }

    pub fn ccli_song_id<S: Into<String>>(content: S) -> Self {
        Self::CCLISongId(content.into())
    }

    pub fn b_notation<S: AsRef<str>>(content: S) -> Self {
        Self::BNotation(BNotation::try_from(content.as_ref()).unwrap_or_default())
    }
}

impl TryFrom<&str> for RawMetadata {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(':').collect::<Vec<&str>>();
        if parts.len() < 2 {
            return Err(());
        }

        match Self::from_keyword_and_content(parts.get(0).unwrap(), parts.get(1).unwrap()) {
            Some(p) => Ok(p),
            None => Err(()),
        }
    }
}

impl TryFrom<&String> for RawMetadata {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.as_str())
    }
}
