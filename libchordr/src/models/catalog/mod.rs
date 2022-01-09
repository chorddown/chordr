use std::slice::Iter;

use serde::{Deserialize, Serialize};

use crate::models::list::ListEntryTrait;
use crate::models::song::Song;
use crate::models::song_id::SongId;
use crate::prelude::RecordTrait;

pub use self::catalog_trait::CatalogTrait;

pub mod catalog_trait;

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

    fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }

    fn iter(&self) -> Iter<'_, Song> {
        self.songs.iter()
    }

    fn revision(&self) -> String {
        self.revision.clone()
    }
}

impl RecordTrait for Catalog {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.revision.clone()
    }
}
