use crate::models::song::Song;
use crate::models::song_data::SongData;
use serde::{Deserialize, Serialize};
use crate::models::song_id::SongId;
use std::slice::Iter;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Catalog {
    revision: String,
    songs: Vec<Song>,
}

impl Catalog {
    pub fn new<S: Into<String>>(revision: S, songs: Vec<Song>) -> Self {
        Self { revision: revision.into(), songs }
    }

    pub fn len(&self) -> usize {
        self.songs.len()
    }

    pub fn revision(&self) -> String {
        self.revision.clone()
    }

    pub fn contains_id<S: Into<SongId>>(&self, song_id: S) -> bool {
        self.get(song_id).is_some()
    }

    pub fn get<S: Into<SongId>>(&self, song_id: S) -> Option<&Song> {
        let song_id = song_id.into();
        self.songs.iter().find(|s| s.id() == song_id)
    }

    pub fn iter(&self) -> Iter<Song> {
        self.songs.iter()
    }
}
