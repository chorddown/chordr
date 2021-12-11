use serde::{Deserialize, Serialize};

use crate::models::chord::Chord;
use crate::models::list::ListEntryTrait;
use crate::models::metadata::*;
use crate::models::song_id::SongIdTrait;

use super::file_type::FileType;
use super::song_data::SongData;
use super::song_id::SongId;

/// Representation of a Song's metadata, used e.g. in the JSON export
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SongMetadata {
    id: SongId,
    title: String,
    #[serde(rename = "type")]
    file_type: FileType,

    // TODO: check if #[serde(flatten)] should be used
    subtitle: Option<String>,
    artist: Option<String>,
    composer: Option<String>,
    lyricist: Option<String>,
    copyright: Option<String>,
    album: Option<String>,
    year: Option<String>,
    key: Option<Chord>,
    original_key: Option<Chord>,
    time: Option<String>,
    tempo: Option<String>,
    duration: Option<String>,
    capo: Option<String>,
    original_title: Option<String>,
    alternative_title: Option<String>,
    ccli_song_id: Option<String>,
    b_notation: BNotation,
}

impl SongMetadata {
    pub fn new(id: SongId, title: String, file_type: FileType) -> Self {
        Self {
            id,
            title,
            file_type,
            subtitle: None,
            artist: None,
            composer: None,
            lyricist: None,
            copyright: None,
            album: None,
            year: None,
            key: None,
            original_key: None,
            time: None,
            tempo: None,
            duration: None,
            capo: None,
            original_title: None,
            alternative_title: None,
            ccli_song_id: None,
            b_notation: Default::default(),
        }
    }

    pub fn new_with_meta_information(
        id: SongId,
        title: String,
        file_type: FileType,
        metadata: &dyn MetadataTrait,
    ) -> Self {
        fn option_to_owned(input: Option<&str>) -> Option<String> {
            input.map(ToOwned::to_owned)
        }
        Self {
            id,
            title,
            file_type,
            subtitle: option_to_owned(metadata.subtitle()),
            artist: option_to_owned(metadata.artist()),
            composer: option_to_owned(metadata.composer()),
            lyricist: option_to_owned(metadata.lyricist()),
            copyright: option_to_owned(metadata.copyright()),
            album: option_to_owned(metadata.album()),
            year: option_to_owned(metadata.year()),
            key: metadata.key().map(ToOwned::to_owned),
            original_key: metadata.original_key().map(ToOwned::to_owned),
            time: option_to_owned(metadata.time()),
            tempo: option_to_owned(metadata.tempo()),
            duration: option_to_owned(metadata.duration()),
            capo: option_to_owned(metadata.capo()),
            original_title: option_to_owned(metadata.original_title()),
            alternative_title: option_to_owned(metadata.alternative_title()),
            ccli_song_id: option_to_owned(metadata.ccli_song_id()),
            b_notation: metadata.b_notation(),
        }
    }
}

impl MetadataTrait for SongMetadata {
    fn title(&self) -> Option<&str> {
        Some(self.title.as_str())
    }

    fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }

    fn artist(&self) -> Option<&str> {
        self.artist.as_deref()
    }

    fn composer(&self) -> Option<&str> {
        self.composer.as_deref()
    }

    fn lyricist(&self) -> Option<&str> {
        self.lyricist.as_deref()
    }

    fn copyright(&self) -> Option<&str> {
        self.copyright.as_deref()
    }

    fn album(&self) -> Option<&str> {
        self.album.as_deref()
    }

    fn year(&self) -> Option<&str> {
        self.year.as_deref()
    }

    fn key(&self) -> Option<&Chord> {
        self.key.as_ref()
    }
    fn original_key(&self) -> Option<&Chord> {
        self.original_key.as_ref()
    }

    fn time(&self) -> Option<&str> {
        self.time.as_deref()
    }

    fn tempo(&self) -> Option<&str> {
        self.tempo.as_deref()
    }

    fn duration(&self) -> Option<&str> {
        self.duration.as_deref()
    }

    fn capo(&self) -> Option<&str> {
        self.capo.as_deref()
    }

    fn original_title(&self) -> Option<&str> {
        self.original_title.as_deref()
    }

    fn alternative_title(&self) -> Option<&str> {
        self.alternative_title.as_deref()
    }

    fn ccli_song_id(&self) -> Option<&str> {
        self.ccli_song_id.as_deref()
    }

    fn b_notation(&self) -> BNotation {
        self.b_notation
    }
}

impl SongIdTrait for SongMetadata {}

impl ListEntryTrait for SongMetadata {
    type Id = SongId;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl SongData for SongMetadata {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn file_type(&self) -> FileType {
        self.file_type
    }
}
