use crate::events::{SetlistEvent, SortingChange};
use libchordr::prelude::{Setlist, SetlistEntry, SongId, SongSettings};

pub trait SetlistHandler {
    /// Handle the given [`Setlist`] related event
    fn handle_setlist_event(&mut self, event: SetlistEvent) -> ();

    /// Add a new entry to the [`Setlist`]
    fn setlist_add(&mut self, song: SetlistEntry) -> ();

    /// Remove an entry from the [`Setlist`]
    fn setlist_remove(&mut self, song_id: SongId) -> ();

    /// Invoked when the [`SongSettings`] for the Song with the given [`SongId`] changed
    fn setlist_settings_changed(&mut self, song_id: SongId, song_settings: SongSettings) -> ();

    /// Replace the locally stored [`Setlist`]
    fn setlist_replace(&mut self, setlist: Setlist) -> ();

    /// Invoked when the sorting of entries in the [`Setlist`] changed
    fn setlist_sorting_changed(&mut self, sorting_change: SortingChange) -> ();

    /// Load the [`Setlist`] from the persistent storage
    fn fetch_setlist(&mut self);

    /// Commit the [`Setlist`] to the persistent storage
    fn commit_changes(&mut self);
}
