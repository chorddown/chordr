use super::SongListTrait;
use crate::error::Result;
use crate::models::list::{List, ListEntryTrait, ListError, ListTrait};
use crate::models::song_id::SongIdTrait;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::ops;
use std::slice::Iter;
use std::vec::IntoIter;

/// A generic set of Songs identified by their [SongId]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SongList<S: SongIdTrait>(List<S>);

impl<S> SongList<S>
where
    S: SongIdTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self { 0: List::new() }
    }
    pub fn with_entries(entries: Vec<S>) -> Self {
        Self {
            0: List::from(entries),
        }
    }

    pub fn iter(&self) -> Iter<'_, S> {
        self.0.iter()
    }
}

impl<S> From<Vec<S>> for SongList<S>
where
    S: SongIdTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    fn from(list: Vec<S>) -> Self {
        SongList { 0: list.into() }
    }
}

impl<S> SongListTrait for SongList<S>
where
    S: SongIdTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
}

impl<S> ListTrait for SongList<S>
where
    S: SongIdTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    type Item = S;

    fn get(&self, id: <<Self as ListTrait>::Item as ListEntryTrait>::Id) -> Option<&Self::Item> {
        self.0.get(id)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add the given [SongData] instance to the [SongList] if it's [SongId] is not already registered
    fn add(&mut self, song: S) -> Result<(), ListError> {
        self.0.add(song)
    }

    /// Replace the given [SongData] instance in the [SongList]
    fn replace(&mut self, song: S) -> Result<(), ListError> {
        self.0.replace(song)
    }

    /// Remove the entry with the given [SongId] from the [SongList]
    fn remove_by_id(
        &mut self,
        song_id: <<Self as ListTrait>::Item as ListEntryTrait>::Id,
    ) -> Result<(), ListError> {
        self.0.remove_by_id(song_id)
    }

    /// Move the entry from one index to another one
    fn move_entry(&mut self, from: usize, to: usize) -> Result<(), ListError> {
        self.0.move_entry(from, to)
    }

    fn position(&mut self, id: <<Self as ListTrait>::Item as ListEntryTrait>::Id) -> Option<usize> {
        self.0.position(id)
    }
}

impl<S: SongIdTrait + PartialEq> PartialEq for SongList<S>
where
    S: ListEntryTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S> ops::Index<usize> for SongList<S>
where
    S: SongIdTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<S> IntoIterator for SongList<S>
where
    S: SongIdTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    type Item = S;
    type IntoIter = IntoIter<S>;

    #[inline]
    fn into_iter(self) -> IntoIter<S> {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::song_id::SongId;

    #[derive(Debug, PartialOrd, PartialEq, Clone, Serialize, Deserialize)]
    struct TestItem(usize);

    impl SongIdTrait for TestItem {}

    impl ListEntryTrait for TestItem {
        type Id = SongId;
        fn id(&self) -> SongId {
            SongId::new(format!("{}", self.0))
        }
    }

    #[test]
    fn move_entry_test() {
        let mut list = SongList::from(vec![
            TestItem(0),
            TestItem(1),
            TestItem(2),
            TestItem(3),
            TestItem(4),
        ]);
        assert!(list.move_entry(1, 3).is_ok());
        assert_eq!(list[0], TestItem(0));
        assert_eq!(list[1], TestItem(2));
        assert_eq!(list[3], TestItem(1));
    }

    #[test]
    fn move_entry_boundary_test() {
        let mut list = SongList::from(vec![
            TestItem(0),
            TestItem(1),
            TestItem(2),
            TestItem(3),
            TestItem(4),
        ]);
        assert!(list.move_entry(0, 4).is_ok());
        assert_eq!(list[0], TestItem(1));
        assert_eq!(list[4], TestItem(0));
    }

    #[test]
    fn for_test() {
        let vector = vec![
            TestItem(0),
            TestItem(1),
            TestItem(2),
            TestItem(3),
            TestItem(4),
        ];
        let list = SongList::from(vector.clone());
        let mut count = 0;
        for entry in list {
            assert_eq!(vector[count], entry);
            count += 1;
        }
        assert_eq!(count, 5);
    }
}
