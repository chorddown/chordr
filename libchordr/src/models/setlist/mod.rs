mod setlist_entry;

pub use self::setlist_entry::SetlistEntry;
use crate::error::{Error, Result};
use crate::models::song_id::{SongId, SongIdTrait};
use serde::Deserialize;
use serde::Serialize;
use std::{mem, ops};
use std::slice::Iter;

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

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Add the given [SongData] instance to the [Setlist] if it's [SongId] is not already registered
    pub fn add(&mut self, song: S) -> Result<()> {
        if !self.contains(&song) {
            self.0.push(song);
            Ok(())
        } else {
            Err(Error::setlist_error(format!(
                "Song {} is already in set-list",
                song.id()
            )))
        }
    }

    /// Replace the given [SongData] instance in the [Setlist]
    pub fn replace(&mut self, song: S) -> Result<()> {
        let song_id = song.id();
        match self.position(song_id.as_str()) {
            Some(pos) => {
                mem::replace(&mut self.0[pos], song);
                Ok(())
            }
            None => Err(Error::setlist_error(format!(
                "Could not find song {} in set-list",
                song_id
            ))),
        }
    }

    /// Remove the entry with the given [SongId] from the [Setlist]
    pub fn remove_by_id<I: AsRef<str>>(&mut self, song_id: I) -> Result<()> {
        let song_id = song_id.as_ref();
        match self.position(song_id) {
            Some(pos) => {
                self.0.remove(pos);
                Ok(())
            }
            None => Err(Error::setlist_error(format!(
                "Could not find song {} in set-list",
                song_id
            ))),
        }
    }

    /// Move the entry from one index to another one
    pub fn move_entry(&mut self, from: usize, to: usize) -> Result<(), Error> {
        let length = self.0.len();
        if from >= length {
            return Err(Error::setlist_error(format!(
                "Invalid 'from' value {} given. Length is {}",
                from,
                length,
            )));
        }
        if to >= length {
            return Err(Error::setlist_error(format!(
                "Invalid 'to' value {} given. Length is {}",
                to,
                length,
            )));
        }
        let item = self.0.remove(from);
        self.0.insert(to, item);
        Ok(())
    }

    /// Get the position of the entry with the given [SongId]
    fn position<I: AsRef<str>>(&mut self, song_id: I) -> Option<usize> {
        let song_id = song_id.as_ref();
        self.0.iter().position(|s| s.id().as_str() == song_id)
    }

    /// Remove the entry with the matching [SongId] from the [Setlist]
    pub fn remove(&mut self, song: &S) -> Result<()> {
        self.remove_by_id(song.id().as_str())
    }

    pub fn iter(&self) -> Iter<'_, S> {
        self.0.iter()
    }
}

impl<S: SongIdTrait + PartialEq> PartialEq for Setlist<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: SongIdTrait> ops::Index<usize> for Setlist<S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialOrd, PartialEq)]
    struct TestItem(usize);

    impl SongIdTrait for TestItem {
        fn id(&self) -> SongId {
            SongId::new(format!("{}", self.0))
        }
    }

    #[test]
    fn move_entry_test() {
        let mut list = Setlist(vec![TestItem(0), TestItem(1), TestItem(2), TestItem(3), TestItem(4)]);
        list.move_entry(1, 3);
        assert_eq!(list[0], TestItem(0));
        assert_eq!(list[1], TestItem(2));
        assert_eq!(list[3], TestItem(1));
    }

    #[test]
    fn move_entry_boundary_test() {
        let mut list = Setlist(vec![TestItem(0), TestItem(1), TestItem(2), TestItem(3), TestItem(4)]);
        list.move_entry(0, 4);
        assert_eq!(list[0], TestItem(1));
        assert_eq!(list[4], TestItem(0));
    }
}
