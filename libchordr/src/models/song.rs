use crate::models::file_type::FileType;
use crate::models::song_data::SongData;
use crate::models::song_meta::SongMeta;
use serde::Deserialize;
use serde::Serialize;

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

impl SongData for Song {
    fn id(&self) -> String {
        self.meta.id()
    }

    fn title(&self) -> String {
        self.meta.title()
    }

    fn file_type(&self) -> FileType {
        self.meta.file_type()
    }
}
