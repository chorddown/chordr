use super::SortingChange;
use crate::EventTrait;
use libchordr::prelude::{Setlist, SetlistEntry, SongId, SongSettings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SetlistEvent {
    /// Append a new Entry to the [`Setlist`]
    Add(SetlistEntry),

    /// Remove an Entry from the [`Setlist`]
    Remove(SongId),

    /// Change [`Settings`] for a Song
    SettingsChange(SongId, SongSettings),

    /// Replace the complete [`Setlist`] with the given one
    Replace(Setlist),

    /// Move one Entry to another position in the [`Setlist`]
    SortingChange(SortingChange),
}

impl EventTrait for SetlistEvent {}
