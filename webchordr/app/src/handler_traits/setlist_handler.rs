use libchordr::prelude::{Setlist, SetlistEntry, SongId, SongSettings};
use webchordr_events::{SetlistEvent, SortingChange};

pub trait SetlistHandler {
    /// Handle the given [`Setlist`] related event
    fn handle_setlist_event(&mut self, event: SetlistEvent);

    /// Add a new [`Setlist`] to the persistent storage
    fn add(&mut self, setlist: Setlist);

    /// Delete a [`Setlist`] from the persistent storage
    fn delete(&mut self, setlist: Setlist);

    /// Update a [`Setlist`] in the persistent storage
    fn update(&mut self, setlist: Setlist);

    /// Load the [`Setlist`] from the persistent storage
    fn fetch_setlists(&mut self);

    /// Add a new entry to the [`Setlist`]
    fn setlist_entry_add(&mut self, song: SetlistEntry);

    /// Remove an entry from the [`Setlist`]
    fn setlist_entry_remove(&mut self, song_id: SongId);

    /// Invoked when the [`SongSettings`] for the Song with the given [`SongId`] changed
    fn setlist_settings_changed(&mut self, song_id: SongId, song_settings: SongSettings);

    /// Replace the locally stored [`Setlist`]
    fn setlist_replace(&mut self, setlist: Setlist);

    /// Set the given setlist as the as the currently active [`Setlist`]
    fn set_current_setlist(&mut self, setlist: Setlist);

    /// Invoked when the sorting of entries in the [`Setlist`] changed
    fn setlist_sorting_changed(&mut self, sorting_change: SortingChange);

    /// Load the [`Setlist`] from the persistent storage
    fn fetch_setlist(&mut self);

    /// Commit the [`Setlist`] to the persistent storage
    fn commit_changes(&mut self);
}
