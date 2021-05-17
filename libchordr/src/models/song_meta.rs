use serde::{Deserialize, Serialize};

use crate::models::list::ListEntryTrait;
use crate::models::meta::*;
use crate::models::song_id::SongIdTrait;

use super::song_data::SongData;
use super::song_id::SongId;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SongMeta {
    id: SongId,
    title: String,

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
    original_title: Option<String>,
    alternative_title: Option<String>,
    ccli_song_id: Option<String>,
    b_notation: BNotation,
}

impl SongMeta {
    pub fn new(id: SongId, title: String) -> Self {
        Self {
            id,
            title,
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
            original_title: None,
            alternative_title: None,
            ccli_song_id: None,
            b_notation: Default::default(),
        }
    }

    pub fn new_with_meta_information(id: SongId, title: String, meta: &dyn MetaTrait) -> Self {
        Self {
            id,
            title,
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
            original_title: meta.original_title(),
            alternative_title: meta.alternative_title(),
            ccli_song_id: meta.ccli_song_id(),
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

    fn original_title(&self) -> Option<String> {
        self.original_title.as_ref().cloned()
    }

    fn alternative_title(&self) -> Option<String> {
        self.alternative_title.as_ref().cloned()
    }

    fn ccli_song_id(&self) -> Option<String> {
        self.ccli_song_id.as_ref().cloned()
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
}
