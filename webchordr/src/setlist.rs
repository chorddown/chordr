use libchordr::prelude::{Song, SongData, Error, Result};

use serde::Deserialize;
use serde::Serialize;
use std::slice::Iter;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Setlist(Vec<Song>);

impl Setlist {
    pub fn new() -> Self {
        Self { 0: Vec::new() }
    }

    pub fn contains<S: SongData>(&self, song: &S) -> bool {
        let song_id = song.id();
        self.0.iter().find(|s| s.id() == song_id).is_some()
    }

    pub fn add(&mut self, song: Song) {
        self.0.push(song)
    }

    pub fn remove(&mut self, song: &Song) -> Result<()> {
        let song_id = song.id();
        match self.0.iter().position(|s| s.id() == song_id) {
            Some(pos) => {
                self.0.remove(pos);
                Ok(())
            }
            None => {
                Err(Error::unknown_error(format!("Could not find song {} in set-list", song_id)))
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, Song> {
        self.0.iter()
    }
}

impl PartialEq for Setlist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
