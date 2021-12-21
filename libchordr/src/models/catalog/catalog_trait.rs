use std::slice::Iter;

use crate::models::song_data::SongData;
use crate::models::song_id::SongId;

pub trait CatalogTrait<E: SongData> {
    /// Return the song with the given `SongId` from the `Catalog`
    fn get<S: Into<SongId>>(&self, song_id: S) -> Option<&E>;

    /// Return the number of songs in the `Catalog`
    fn len(&self) -> usize;

    /// Return if the `Catalog` is empty
    fn is_empty(&self) -> bool;

    /// Return an iterator over the songs of the `Catalog`
    fn iter(&self) -> Iter<'_, E>;

    /// Return the revision of the `Catalog`
    fn revision(&self) -> String;

    /// Return if the `Catalog` contains a song with the given identifier
    fn contains_id<S: Into<SongId>>(&self, song_id: S) -> bool {
        self.get(song_id).is_some()
    }
}
