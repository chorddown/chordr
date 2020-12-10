use super::file_type::FileType;
use super::song_data::SongData;
use super::song_id::SongId;
use crate::models::list::ListEntryTrait;
use crate::models::meta::*;
use crate::models::song_id::SongIdTrait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SongMeta {
    id: SongId,
    title: String,
    #[serde(rename = "type")]
    file_type: FileType,

    subtitle: Option<String>,
    artist: Option<String>,
    composer: Option<String>,
    lyricist: Option<String>,
    copyright: Option<String>,
    album: Option<String>,
    year: Option<String>,
    key: Option<String>,
    time: Option<String>,
    tempo: Option<String>,
    duration: Option<String>,
    capo: Option<String>,
    b_notation: BNotation,
}

impl SongMeta {
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
            time: None,
            tempo: None,
            duration: None,
            capo: None,
            b_notation: Default::default(),
        }
    }

    pub fn new_with_meta_information(
        id: SongId,
        title: String,
        file_type: FileType,
        meta: &dyn MetaTrait,
    ) -> Self {
        Self {
            id,
            title,
            file_type,
            subtitle: meta.subtitle(),
            artist: meta.artist(),
            composer: meta.composer(),
            lyricist: meta.lyricist(),
            copyright: meta.copyright(),
            album: meta.album(),
            year: meta.year(),
            key: meta.key(),
            time: meta.time(),
            tempo: meta.tempo(),
            duration: meta.duration(),
            capo: meta.capo(),
            b_notation: meta.b_notation(),
        }
    }
}

impl MetaTrait for SongMeta {
    fn title(&self) -> Option<String> {
        Some(self.title.clone())
    }

    fn subtitle(&self) -> Option<String> {
        self.subtitle.as_ref().cloned()
    }

    fn artist(&self) -> Option<String> {
        self.artist.as_ref().cloned()
    }

    fn composer(&self) -> Option<String> {
        self.composer.as_ref().cloned()
    }

    fn lyricist(&self) -> Option<String> {
        self.lyricist.as_ref().cloned()
    }

    fn copyright(&self) -> Option<String> {
        self.copyright.as_ref().cloned()
    }

    fn album(&self) -> Option<String> {
        self.album.as_ref().cloned()
    }

    fn year(&self) -> Option<String> {
        self.year.as_ref().cloned()
    }

    fn key(&self) -> Option<String> {
        self.key.as_ref().cloned()
    }

    fn time(&self) -> Option<String> {
        self.time.as_ref().cloned()
    }

    fn tempo(&self) -> Option<String> {
        self.tempo.as_ref().cloned()
    }

    fn duration(&self) -> Option<String> {
        self.duration.as_ref().cloned()
    }

    fn capo(&self) -> Option<String> {
        self.capo.as_ref().cloned()
    }

    fn b_notation(&self) -> BNotation {
        self.b_notation
    }
}

impl SongIdTrait for SongMeta {}

impl ListEntryTrait for SongMeta {
    type Id = SongId;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl SongData for SongMeta {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn file_type(&self) -> FileType {
        self.file_type
    }
}
