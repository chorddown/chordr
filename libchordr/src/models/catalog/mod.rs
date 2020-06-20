pub mod catalog_trait;

pub use self::catalog_trait::CatalogTrait;
use crate::models::list::ListEntryTrait;
use crate::models::song::Song;
use crate::models::song_id::SongId;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Catalog {
    revision: String,
    songs: Vec<Song>,
}

impl Catalog {
    pub fn new<S: Into<String>>(revision: S, songs: Vec<Song>) -> Self {
        Self {
            revision: revision.into(),
            songs,
        }
    }
}

impl CatalogTrait<Song> for Catalog {
    fn get<S: Into<SongId>>(&self, song_id: S) -> Option<&Song> {
        let song_id = song_id.into();
        self.songs.iter().find(|s| s.id() == song_id)
    }

    fn len(&self) -> usize {
        self.songs.len()
    }

    fn iter(&self) -> Iter<Song> {
        self.songs.iter()
    }

    fn revision(&self) -> String {
        self.revision.clone()
    }
}
