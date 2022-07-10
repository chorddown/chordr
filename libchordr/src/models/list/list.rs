use std::fmt::Debug;
use std::slice::Iter;
use std::vec::IntoIter;
use std::{mem, ops};

use serde::{Deserialize, Serialize};

use crate::models::list::list_trait::ListEntryTrait;

use super::list_error::ListError;
use super::list_trait::ListTrait;

/// A generic list of entries
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct List<S: ListEntryTrait>(Vec<S>);

impl<S> List<S>
where
    S: ListEntryTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn iter(&self) -> Iter<'_, S> {
        self.0.iter()
    }
}

impl<S: ListEntryTrait> From<Vec<S>> for List<S> {
    fn from(items: Vec<S>) -> Self {
        Self(items)
    }
}

impl<S: ListEntryTrait> FromIterator<S> for List<S> {
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let mut c = Vec::new();

        for i in iter {
            c.push(i);
        }

        List::from(c)
    }
}

impl<S> ListTrait for List<S>
where
    S: ListEntryTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    type Item = S;

    fn get(&self, song_id: <Self::Item as ListEntryTrait>::Id) -> Option<&S> {
        self.0.iter().find(|s| s.id() == song_id)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn add(&mut self, item: Self::Item) -> Result<(), ListError> {
        if !self.contains(&item) {
            self.0.push(item);
            Ok(())
        } else {
            Err(ListError::AlreadyInList)
        }
    }

    fn replace(&mut self, item: Self::Item) -> Result<(), ListError> {
        let item_id = item.id();
        match self.position(item_id) {
            Some(pos) => {
                let _ = mem::replace(&mut self.0[pos], item);
                Ok(())
            }
            None => Err(ListError::NotFound),
        }
    }

    fn remove_by_id(&mut self, id: <Self::Item as ListEntryTrait>::Id) -> Result<(), ListError> {
        match self.position(id) {
            Some(pos) => {
                self.0.remove(pos);
                Ok(())
            }
            None => Err(ListError::NotFound),
        }
    }

    fn move_entry(&mut self, from: usize, to: usize) -> Result<(), ListError> {
        let length = self.0.len();
        if from >= length {
            return Err(ListError::MoveError(format!(
                "Invalid 'from' value {} given. Length is {}",
                from, length,
            )));
        }
        if to >= length {
            return Err(ListError::MoveError(format!(
                "Invalid 'to' value {} given. Length is {}",
                to, length,
            )));
        }
        let item = self.0.remove(from);
        self.0.insert(to, item);
        Ok(())
    }

    fn position(&mut self, id: <Self::Item as ListEntryTrait>::Id) -> Option<usize> {
        self.0.iter().position(|s| s.id() == id)
    }
}

impl<S> Default for List<S>
where
    S: ListEntryTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S: PartialEq> PartialEq for List<S>
where
    S: ListEntryTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S> ops::Index<usize> for List<S>
where
    S: ListEntryTrait + Serialize + Debug + Clone,
    S: for<'a> Deserialize<'a>,
{
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<S: Serialize + Debug + Clone> IntoIterator for List<S>
where
    S: for<'a> Deserialize<'a>,
    S: ListEntryTrait,
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
    use crate::models::list::list_trait::ListEntryTrait;
    use crate::models::song_id::SongId;

    use super::*;

    #[derive(Debug, PartialOrd, PartialEq, Clone, Serialize, Deserialize)]
    struct TestItem(usize);

    impl ListEntryTrait for TestItem {
        type Id = SongId;

        fn id(&self) -> SongId {
            SongId::new(format!("{}", self.0))
        }
    }

    #[test]
    fn move_entry_test() {
        let mut list = List(vec![
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
    fn move_entry_to_same_position_test() {
        let mut list = List(vec![
            TestItem(0),
            TestItem(1),
            TestItem(2),
            TestItem(3),
            TestItem(4),
        ]);
        assert!(list.move_entry(1, 1).is_ok());
        assert_eq!(list[0], TestItem(0));
        assert_eq!(list[1], TestItem(1));
        assert_eq!(list[3], TestItem(3));
        assert_eq!(list[4], TestItem(4));
    }

    #[test]
    fn move_entry_boundary_test() {
        let mut list = List(vec![
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
        let list = List(vector.clone());
        let mut count = 0;
        for entry in list.clone() {
            assert_eq!(vector[count], entry);
            count += 1;
        }
        assert_eq!(count, 5);

        assert_eq!(list.into_iter().collect::<Vec<TestItem>>().len(), 5)
    }
}
