use crate::error::Result;
use crate::models::song_id::{SongId, SongIdTrait};
use std::slice::Iter;

pub trait SongListTrait {
    type Item: SongIdTrait;
    fn contains<D: SongIdTrait>(&self, song: &D) -> bool {
        let song_id = song.id();
        self.get(song_id).is_some()
    }

    fn contains_id(&self, song_id: SongId) -> bool {
        self.get(song_id).is_some()
    }

    fn get(&self, song_id: SongId) -> Option<&Self::Item> {
        self.iter().find(|s| s.id() == song_id)
    }

    /// Return the number of entries in the list
    fn len(&self) -> usize;

    /// Add the given [SongData] instance to the [SongList] if it's [SongId] is not already registered
    fn add(&mut self, song: Self::Item) -> Result<()>;

    /// Replace the given [SongData] instance in the [SongList]
    fn replace(&mut self, song: Self::Item) -> Result<()>;

    /// Remove the entry with the given [SongId] from the [SongList]
    fn remove_by_id<I: AsRef<str>>(&mut self, song_id: I) -> Result<()>;

    /// Move the entry from one index to another one
    fn move_entry(&mut self, from: usize, to: usize) -> Result<()>;

    /// Remove the entry with the matching [SongId] from the [SongList]
    fn remove(&mut self, song: &Self::Item) -> Result<()> {
        self.remove_by_id(song.id().as_str())
    }

    /// Get the position of the entry with the given [SongId]
    fn position<I: AsRef<str>>(&mut self, song_id: I) -> Option<usize>;

    /// Return an iterator over the entries
    fn iter(&self) -> Iter<'_, Self::Item>;
}
