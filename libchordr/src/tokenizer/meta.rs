use crate::models::meta::BNotation;
use std::convert::TryFrom;

/// Meta information gathered during tokenization
///
/// Some, but not all meta information can be retrieved during tokenization. The song title e.g.
/// will be determined by the Parser
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Meta {
    Artist(String),
    Composer(String),
    Lyricist(String),
    Copyright(String),
    Album(String),
    Year(String),
    Key(String),
    Time(String),
    Tempo(String),
    Duration(String),
    Capo(String),
    BNotation(BNotation),
}

impl Meta {
    fn from_keyword_and_content(word: &str, content: &str) -> Option<Self> {
        let content = content.trim();
        match word.trim().to_lowercase().as_str() {
            "artist" => Some(Self::artist(content)),
            "composer" => Some(Self::composer(content)),
            "lyricist" => Some(Self::lyricist(content)),
            "copyright" => Some(Self::copyright(content)),
            "album" => Some(Self::album(content)),
            "year" => Some(Self::year(content)),
            "key" => Some(Self::key(content)),
            "time" => Some(Self::time(content)),
            "tempo" => Some(Self::tempo(content)),
            "duration" => Some(Self::duration(content)),
            "capo" => Some(Self::capo(content)),
            "bnotation" | "b_notation" | "b notation" | "b-notation" => {
                Some(Self::b_notation(content))
            }
            _ => None,
        }
    }

    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Artist(_) => "Artist",
            Self::Composer(_) => "Composer",
            Self::Lyricist(_) => "Lyricist",
            Self::Copyright(_) => "Copyright",
            Self::Album(_) => "Album",
            Self::Year(_) => "Year",
            Self::Key(_) => "Key",
            Self::Time(_) => "Time",
            Self::Tempo(_) => "Tempo",
            Self::Duration(_) => "Duration",
            Self::Capo(_) => "Capo",
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
            Self::Time(c) => c,
            Self::Tempo(c) => c,
            Self::Duration(c) => c,
            Self::Capo(c) => c,
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

    pub fn time<S: Into<String>>(content: S) -> Self {
        Self::Time(content.into())
    }

    pub fn tempo<S: Into<String>>(content: S) -> Self {
        Self::Tempo(content.into())
    }

    pub fn duration<S: Into<String>>(content: S) -> Self {
        Self::Duration(content.into())
    }

    pub fn capo<S: Into<String>>(content: S) -> Self {
        Self::Capo(content.into())
    }

    pub fn b_notation<S: AsRef<str>>(content: S) -> Self {
        Self::BNotation(match BNotation::try_from(content.as_ref()) {
            Ok(n) => n,
            Err(_) => Default::default(),
        })
    }
}

impl TryFrom<&str> for Meta {
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

impl TryFrom<&String> for Meta {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.as_str())
    }
}
