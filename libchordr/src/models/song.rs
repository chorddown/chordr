use serde::{Deserialize, Serialize};

use crate::models::file_type::FileType;
use crate::models::list::ListEntryTrait;
use crate::models::song_data::SongData;
use crate::models::song_id::{SongId, SongIdTrait};
use crate::models::song_meta::SongMeta;
use crate::models::song_sorting;
use crate::models::song_sorting::SongSorting;

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

    fn file_type(&self) -> FileType {
        self.meta.file_type()
    }
}

impl SongSorting<Song> for Vec<Song> {
    fn sort_by_title(mut self) -> Vec<Song> {
        song_sorting::sort_by_title(&mut self).to_vec()
    }
}

// Implement SongData etc. for &Song
impl ListEntryTrait for &Song {
    type Id = SongId;

    fn id(&self) -> Self::Id {
        self.meta.id()
    }
}

impl SongIdTrait for &Song {}

impl SongData for &Song {
    fn title(&self) -> String {
        self.meta.title()
    }

    fn file_type(&self) -> FileType {
        self.meta.file_type()
    }
}

impl<'a> SongSorting<&'a Song> for Vec<&'a Song> {
    fn sort_by_title(mut self) -> Self {
        song_sorting::sort_by_title(&mut self).to_vec()
    }
}
