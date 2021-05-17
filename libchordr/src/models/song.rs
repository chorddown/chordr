use crate::models::list::ListEntryTrait;
use crate::models::song_data::SongData;
use crate::models::song_id::{SongId, SongIdTrait};
use crate::models::song_meta::SongMeta;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Song {
    meta: SongMeta,
    src: String,
}

impl Song {
    pub fn new<S: Into<String>>(meta: SongMeta, src: S) -> Self {
        Self {
            meta,
            src: src.into(),
        }
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn meta(&self) -> &SongMeta {
        &self.meta
    }
}

impl SongIdTrait for Song {}

impl ListEntryTrait for Song {
    type Id = SongId;

    fn id(&self) -> Self::Id {
        self.meta.id()
    }
}

impl SongData for Song {
    fn title(&self) -> String {
        self.meta.title()
    }
}
