use serde::Deserialize;
use serde::Serialize;
use std::slice::Iter;
use crate::models::song_data::SongData;
use crate::models::song_id::{SongId, SongIdTrait};
use crate::error::{Error, Result};

/// A generic set of Songs identified by their [SongId]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Setlist<S: SongIdTrait>(Vec<S>);

impl<S: SongIdTrait> Setlist<S> {
    pub fn new() -> Self {
        Self { 0: Vec::new() }
    }

    pub fn contains<D: SongIdTrait>(&self, song: &D) -> bool {
        let song_id = song.id();
        self.get(song_id).is_some()
    }

    pub fn contains_id(&self, song_id: SongId) -> bool {
        self.get(song_id).is_some()
    }

    pub fn get(&self, song_id: SongId) -> Option<&S> {
        self.0.iter().find(|s| s.id() == song_id)
    }

    /// Add the given [SongData] instance to the [Setlist] if it's [SongId] is not already registered
    pub fn add(&mut self, song: S) -> Result<(), ()> {
        if !self.contains(&song) {
            self.0.push(song);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Replace the given [SongData] instance in the [Setlist]
    pub fn replace(&mut self, song: S) -> Result<(), ()> {
        unimplemented!()
    }

    /// Remove the entry with the given [SongId] from the [Setlist]
    pub fn remove_by_id(&mut self, song_id: SongId) -> Result<()> {
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

    /// Remove the entry with the matching [SongId] from the [Setlist]
    pub fn remove(&mut self, song: &S) -> Result<()> {
        self.remove_by_id(song.id())
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
