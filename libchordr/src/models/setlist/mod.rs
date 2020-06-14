mod setlist_entry;

pub use self::setlist_entry::SetlistEntry;
use crate::error::Result;
use crate::models::song_id::{SongId, SongIdTrait};
use crate::models::song_list::{SongList, SongListTrait};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::ops;
use std::slice::Iter;

/// A generic set of Songs identified by their [SongId]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Setlist {
    name: String,
    id: i32,
    songs: SongList<SetlistEntry>,
    gig_date: DateTime<Utc>,
    creation_date: DateTime<Utc>,
    modification_date: DateTime<Utc>,
}

impl Setlist {
    pub fn default() -> Self {
        let now = Utc::now();

        Self {
            name: "".to_string(),
            id: 0,
            songs: SongList::new(),
            gig_date: now,
            creation_date: now,
            modification_date: now,
        }
    }

    pub fn new<S: Into<String>>(
        name: S,
        id: i32,
        gig_date: DateTime<Utc>,
        creation_date: DateTime<Utc>,
        modification_date: DateTime<Utc>,
        songs: Vec<SetlistEntry>,
    ) -> Self {
        Self {
            name: name.into(),
            id,
            songs: SongList::with_entries(songs),
            gig_date,
            creation_date,
            modification_date,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn gig_date(&self) -> DateTime<Utc> {
        self.gig_date
    }

    pub fn creation_date(&self) -> DateTime<Utc> {
        self.creation_date
    }

    pub fn modification_date(&self) -> DateTime<Utc> {
        self.modification_date
    }
}

impl SongListTrait for Setlist {
    type Item = SetlistEntry;

    fn contains<D: SongIdTrait>(&self, song: &D) -> bool {
        self.songs.contains(song)
    }

    fn contains_id(&self, song_id: SongId) -> bool {
        self.songs.contains_id(song_id)
    }

    fn get(&self, song_id: SongId) -> Option<&SetlistEntry> {
        self.songs.get(song_id)
    }

    fn len(&self) -> usize {
        self.songs.len()
    }

    fn add(&mut self, song: SetlistEntry) -> Result<()> {
        self.songs.add(song)
    }

    fn replace(&mut self, song: SetlistEntry) -> Result<()> {
        self.songs.replace(song)
    }

    fn remove_by_id<I: AsRef<str>>(&mut self, song_id: I) -> Result<()> {
        self.songs.remove_by_id(song_id)
    }

    fn move_entry(&mut self, from: usize, to: usize) -> Result<()> {
        self.songs.move_entry(from, to)
    }

    fn position<I: AsRef<str>>(&mut self, song_id: I) -> Option<usize> {
        self.songs.position(song_id)
    }

    fn iter(&self) -> Iter<'_, SetlistEntry> {
        self.songs.iter()
    }
}

impl Setlist {
    #[deprecated]
    pub fn with_entries(songs: Vec<SetlistEntry>) -> Self {
        let mut new = Self::default();
        new.songs = SongList::with_entries(songs);

        new
    }
}

// impl<S: SetlistEntryTrait + SongIdTrait + PartialEq> PartialEq for Setlist<S> {
//     fn eq(&self, other: &Self) -> bool {
//         self.songs == other.songs && self.name == other.name && self.id == other.id
//     }
// }

impl ops::Index<usize> for Setlist {
    type Output = SetlistEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.songs[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::FileType;

    fn entry<S: Into<SongId>>(id: S) -> SetlistEntry {
        let song_id = id.into();
        let title = format!("Song {}", song_id);

        SetlistEntry::new(song_id, FileType::Chorddown, title, None)
    }

    #[test]
    fn move_entry_test() {
        let now = Utc::now();
        let mut list = Setlist::new(
            "Setlist name",
            1,
            now,
            now,
            now,
            vec![entry("0"), entry("1"), entry("2"), entry("3"), entry("4")],
        );
        assert!(list.move_entry(1, 3).is_ok());
        assert_eq!(list[0], entry("0"));
        assert_eq!(list[1], entry("2"));
        assert_eq!(list[3], entry("1"));
    }

    #[test]
    fn move_entry_boundary_test() {
        let now = Utc::now();
        let mut list = Setlist::new(
            "Setlist name",
            1,
            now,
            now,
            now,
            vec![entry("0"), entry("1"), entry("2"), entry("3"), entry("4")],
        );
        assert!(list.move_entry(0, 4).is_ok());
        assert_eq!(list[0], entry("1"));
        assert_eq!(list[4], entry("0"));
    }
}
