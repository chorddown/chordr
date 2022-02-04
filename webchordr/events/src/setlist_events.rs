use serde::{Deserialize, Serialize};

use libchordr::prelude::{Setlist, SetlistEntry, SongId, SongSettings};

use crate::EventTrait;

use super::SortingChange;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SetlistEvent {
    /// Append a new Entry to the [`Setlist`]
    AddEntry(SetlistEntry),

    /// Remove an Entry from the [`Setlist`]
    RemoveEntry(SongId),

    /// Change [`Settings`] for a Song
    SettingsChange(SongId, SongSettings),

    /// Replace the complete [`Setlist`] with the given one
    Replace(Setlist),

    /// Make the given [`Setlist`] the currently loaded one
    SetCurrentSetlist(Setlist),

    /// Move one Entry to another position in the [`Setlist`]
    SortingChange(SortingChange),
}

impl EventTrait for SetlistEvent {}
