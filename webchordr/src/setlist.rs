use libchordr::prelude::{SongData, Error, Result};

use serde::Deserialize;
use serde::Serialize;
use std::slice::Iter;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Setlist<S: SongData>(Vec<S>);

impl<S: SongData> Setlist<S> {
    pub fn new() -> Self {
        Self { 0: Vec::new() }
    }

    pub fn contains<D: SongData>(&self, song: &D) -> bool {
        let song_id = song.id();
        self.0.iter().find(|s| s.id() == song_id).is_some()
    }

    pub fn add(&mut self, song: S) {
        self.0.push(song)
    }

    pub fn remove(&mut self, song: &S) -> Result<()> {
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

    pub fn iter(&self) -> Iter<'_, S> {
        self.0.iter()
    }
}

impl<S: SongData + PartialEq> PartialEq for Setlist<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
