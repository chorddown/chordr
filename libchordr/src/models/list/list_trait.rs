use super::list_error::ListError;
use crate::error::Result;
use std::hash::Hash;

pub trait ListEntryTrait {
    type Id: Hash + Eq;
    fn id(&self) -> Self::Id;
}

pub trait ListTrait {
    type Item: ListEntryTrait;
    fn contains(&self, item: &Self::Item) -> bool {
        self.get(item.id()).is_some()
    }

    fn contains_id(&self, id: <Self::Item as ListEntryTrait>::Id) -> bool {
        self.get(id).is_some()
    }

    fn get(&self, id: <Self::Item as ListEntryTrait>::Id) -> Option<&Self::Item>;

    /// Return the number of entries in the list
    fn len(&self) -> usize;

    /// Return if the list is empty
    fn is_empty(&self) -> bool;

    /// Add the given instance to the [List] if it's [Item::Id] is not already registered
    fn add(&mut self, item: Self::Item) -> Result<(), ListError>;

    /// Replace the given instance in the [List]
    fn replace(&mut self, item: Self::Item) -> Result<(), ListError>;

    /// Remove the entry with the given [Item::Id] from the [List]
    fn remove_by_id(&mut self, id: <Self::Item as ListEntryTrait>::Id) -> Result<(), ListError>;

    /// Remove the entry with the matching [Item::Id] from the [List]
    fn remove(&mut self, item: &Self::Item) -> Result<(), ListError> {
        self.remove_by_id(item.id())
    }

    /// Move the entry from one index to another one
    fn move_entry(&mut self, from: usize, to: usize) -> Result<(), ListError>;

    /// Get the position of the entry with the given [Item::Id]
    fn position(&mut self, id: <Self::Item as ListEntryTrait>::Id) -> Option<usize>;
}
