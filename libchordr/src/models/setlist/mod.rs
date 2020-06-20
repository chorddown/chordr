mod setlist_collection;
mod setlist_entry;

pub use self::setlist_collection::SetlistCollection;
pub use self::setlist_entry::SetlistEntry;
use crate::error::Result;
use crate::models::list::{List, ListEntryTrait, ListError, ListTrait};
use crate::models::song_id::SongId;
use crate::models::song_list::{SongList, SongListTrait};
use crate::models::team::Team;
use crate::models::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::ops;
use std::vec::IntoIter;

/// A generic set of Songs identified by their [SongId]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Setlist {
    name: String,
    id: i32,
    owner: User,
    team: Option<Team>,
    songs: List<SetlistEntry>,
    gig_date: DateTime<Utc>,
    creation_date: DateTime<Utc>,
    modification_date: DateTime<Utc>,
}

impl Setlist {
    pub fn new<S: Into<String>>(
        name: S,
        id: i32,
        owner: User,
        team: Option<Team>,
        gig_date: DateTime<Utc>,
        creation_date: DateTime<Utc>,
        modification_date: DateTime<Utc>,
        songs: Vec<SetlistEntry>,
    ) -> Self {
        Self {
            name: name.into(),
            id,
            owner,
            team,
            songs: List::from(songs),
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

    pub fn owner(&self) -> &User {
        &self.owner
    }

    pub fn team(&self) -> &Option<Team> {
        &self.team
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

    pub fn as_song_list(&self) -> SongList<SetlistEntry> {
        SongList::from(
            self.songs
                .clone()
                .into_iter()
                .collect::<Vec<SetlistEntry>>(),
        )
    }
}

impl SongListTrait for Setlist {
    // fn iter(&self) -> Iter<'_, SetlistEntry> {
    //     // TODO: Fix this
    //     self.as_song_list().iter()
    //     // self.songs.iter()
    // }
}

impl ListTrait for Setlist {
    type Item = SetlistEntry;

    fn contains(&self, song: &Self::Item) -> bool {
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

    fn add(&mut self, song: SetlistEntry) -> Result<(), ListError> {
        self.songs.add(song)
    }

    fn replace(&mut self, song: SetlistEntry) -> Result<(), ListError> {
        self.songs.replace(song)
    }

    fn remove_by_id(
        &mut self,
        id: <<Self as ListTrait>::Item as ListEntryTrait>::Id,
    ) -> Result<(), ListError> {
        self.songs.remove_by_id(id)
    }

    // fn remove(&mut self, item: &Self::Item) -> Result<(), ListError> {
    //     self.remove_by_id(item.id())
    // }
    fn remove(&mut self, item: &<Self as ListTrait>::Item) -> Result<(), ListError> {
        self.remove_by_id(item.id())
    }

    fn move_entry(&mut self, from: usize, to: usize) -> Result<(), ListError> {
        self.songs.move_entry(from, to)
    }

    fn position(
        &mut self,
        song_id: <<Self as ListTrait>::Item as ListEntryTrait>::Id,
    ) -> Option<usize> {
        self.songs.position(song_id)
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

impl IntoIterator for Setlist {
    type Item = SetlistEntry;
    type IntoIter = IntoIter<SetlistEntry>;

    #[inline]
    fn into_iter(self) -> IntoIter<SetlistEntry> {
        self.songs.into_iter()
    }
}

impl AsRef<Setlist> for Setlist {
    fn as_ref(&self) -> &Setlist {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::FileType;
    use crate::test_helpers::get_test_user;

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
            get_test_user(),
            None,
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
            get_test_user(),
            None,
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
