use std::convert::TryFrom;

const TITLE: &str = "title";
const SUBTITLE: &str = "subtitle";
const ARTIST: &str = "artist";
const COMPOSER: &str = "composer";
const LYRICIST: &str = "lyricist";
const COPYRIGHT: &str = "copyright";
const ALBUM: &str = "album";
const YEAR: &str = "year";
const KEY: &str = "key";
const ORIGINAL_KEY: &str = "original-key";
const TIME: &str = "time";
const TEMPO: &str = "tempo";
const DURATION: &str = "duration";
const CAPO: &str = "capo";
const ORIGINAL_TITLE: &str = "original-title";
const ALTERNATIVE_TITLE: &str = "alternative-title";
const CCLI_SONG_ID: &str = "ccli-song-id";
const B_NOTATION: &str = "b-notation";

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MetadataKeyword {
    Title,
    Subtitle,
    Artist,
    Composer,
    Lyricist,
    Copyright,
    Album,
    Year,
    Key,
    OriginalKey,
    Time,
    Tempo,
    Duration,
    Capo,
    OriginalTitle,
    AlternativeTitle,
    CCLISongId,
    BNotation,
}

impl MetadataKeyword {
    #[inline(always)]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Title => TITLE,
            Self::Subtitle => SUBTITLE,
            Self::Artist => ARTIST,
            Self::Composer => COMPOSER,
            Self::Lyricist => LYRICIST,
            Self::Copyright => COPYRIGHT,
            Self::Album => ALBUM,
            Self::Year => YEAR,
            Self::Key => KEY,
            Self::OriginalKey => ORIGINAL_KEY,
            Self::Time => TIME,
            Self::Tempo => TEMPO,
            Self::Duration => DURATION,
            Self::Capo => CAPO,
            Self::OriginalTitle => ORIGINAL_TITLE,
            Self::AlternativeTitle => ALTERNATIVE_TITLE,
            Self::CCLISongId => CCLI_SONG_ID,
            Self::BNotation => B_NOTATION,
        }
    }

    /// Return a descriptive title for the metadata
    #[inline(always)]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Title => "Title",
            Self::Artist => "Artist",
            Self::Composer => "Composer",
            Self::Lyricist => "Lyricist",
            Self::Copyright => "Copyright",
            Self::Album => "Album",
            Self::Year => "Year",
            Self::Key => "Key",
            Self::OriginalKey => "Original Key",
            Self::Time => "Time",
            Self::Tempo => "Tempo",
            Self::Duration => "Duration",
            Self::Subtitle => "Subtitle",
            Self::Capo => "Capo",
            Self::OriginalTitle => "Original Title",
            Self::AlternativeTitle => "Alternative Title",
            Self::CCLISongId => "CCLI Song #",
            Self::BNotation => "B-Notation",
        }
    }
}

impl TryFrom<&str> for MetadataKeyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            ARTIST => Ok(Self::Artist),
            COMPOSER => Ok(Self::Composer),
            LYRICIST => Ok(Self::Lyricist),
            COPYRIGHT => Ok(Self::Copyright),
            ALBUM => Ok(Self::Album),
            YEAR => Ok(Self::Year),
            KEY => Ok(Self::Key),
            ORIGINAL_KEY | "original_key" | "original key" | "originalkey" => Ok(Self::OriginalKey),
            TIME => Ok(Self::Time),
            TEMPO => Ok(Self::Tempo),
            DURATION => Ok(Self::Duration),
            SUBTITLE => Ok(Self::Subtitle),
            CAPO => Ok(Self::Capo),
            ORIGINAL_TITLE | "original_title" | "original title" | "originaltitle" => {
                Ok(Self::OriginalTitle)
            }
            ALTERNATIVE_TITLE | "alternative_title" | "alternative title" => {
                Ok(Self::AlternativeTitle)
            }
            CCLI_SONG_ID | "ccli song #" | "ccli song" | "ccli song id" | "ccli_song_id" => {
                Ok(Self::CCLISongId)
            }
            B_NOTATION | "bnotation" | "b notation" | "b_notation" => Ok(Self::BNotation),
            _ => Err(()),
        }
    }
}
