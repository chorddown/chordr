use std::convert::TryFrom;

use crate::models::meta::{BNotation, Tags};

/// Meta information gathered during tokenization
///
/// Some, but not all meta information can be retrieved during tokenization. The song title e.g.
/// will be determined by the Parser
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Meta {
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
    Tags(Tags),
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
            "original-key" | "original key" | "originalkey" => Some(Self::original_key(content)),
            "time" => Some(Self::time(content)),
            "tempo" => Some(Self::tempo(content)),
            "duration" => Some(Self::duration(content)),
            "subtitle" => Some(Self::subtitle(content)),
            "capo" => Some(Self::capo(content)),
            "original-title" | "original title" | "originaltitle" => {
                Some(Self::original_title(content))
            }
            "alternative-title" | "alternative title" => Some(Self::alternative_title(content)),
            "ccli song #" | "ccli song" | "ccli song id" => Some(Self::ccli_song_id(content)),
            "bnotation" | "b_notation" | "b notation" | "b-notation" => {
                Some(Self::b_notation(content))
            }
            "tags" => Some(Self::tags(content)),
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
            Self::Tags(_) => "Tags",
        }
    }

    pub fn content(&self) -> String {
        match self {
            Self::Artist(c) => c.to_owned(),
            Self::Composer(c) => c.to_owned(),
            Self::Lyricist(c) => c.to_owned(),
            Self::Copyright(c) => c.to_owned(),
            Self::Album(c) => c.to_owned(),
            Self::Year(c) => c.to_owned(),
            Self::Key(c) => c.to_owned(),
            Self::OriginalKey(c) => c.to_owned(),
            Self::Time(c) => c.to_owned(),
            Self::Tempo(c) => c.to_owned(),
            Self::Duration(c) => c.to_owned(),
            Self::Subtitle(c) => c.to_owned(),
            Self::Capo(c) => c.to_owned(),
            Self::OriginalTitle(c) => c.to_owned(),
            Self::AlternativeTitle(c) => c.to_owned(),
            Self::CCLISongId(c) => c.to_owned(),
            Self::BNotation(c) => c.to_string(),
            Self::Tags(c) => c.to_string(),
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
        Self::BNotation(match BNotation::try_from(content.as_ref()) {
            Ok(n) => n,
            Err(_) => Default::default(),
        })
    }

    pub fn tags<S: AsRef<str>>(content: S) -> Self {
        Self::Tags(match Tags::try_from(content.as_ref()) {
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
