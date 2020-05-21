use crate::events::{SetlistEvent, SortingChange};
use libchordr::prelude::{Setlist, SetlistEntry, SongId};
use yew::services::StorageService;

pub trait SetlistHandler {
    fn handle_setlist_event(&mut self, event: SetlistEvent) -> ();

    fn setlist_add(&mut self, song: SetlistEntry) -> ();

    fn setlist_remove(&mut self, song_id: SongId) -> ();

    fn setlist_replace(&mut self, setlist: Setlist<SetlistEntry>) -> ();

    fn setlist_sorting_changed(&mut self, sorting_change: SortingChange) -> ();

    fn get_setlist(storage_service: &StorageService) -> Setlist<SetlistEntry>;
}
