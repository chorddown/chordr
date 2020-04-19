use serde::{Deserialize, Serialize};
use super::SortingChange;
use libchordr::prelude::{SetlistEntry, SongId, Setlist};
use crate::events::EventTrait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SetlistEvent {
    /// Append a new Entry to the Setlist
    Add(SetlistEntry),

    /// Remove an Entry from the Setlist
    Remove(SongId),

    /// Replace the complete Setlist with the given one
    Replace(Setlist<SetlistEntry>),

    /// Move one Entry to another position in the Setlist
    SortingChange(SortingChange),
}

impl EventTrait for SetlistEvent {}
