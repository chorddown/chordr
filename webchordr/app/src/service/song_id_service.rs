use libchordr::prelude::{Catalog, CatalogTrait, SongId};
use percent_encoding::percent_decode_str;

pub struct SongIdService {}

impl SongIdService {
    pub fn new() -> Self {
        Self {}
    }

    /// Return the normalized `SongId`
    ///
    /// If `song_id` can not be found in the catalog, the ID will be "percent-decoded" and the
    /// resulting `SongId` will be searched in the catalog. If the decoded ID was found, it will be
    /// returned. If neither the original nor the decoded ID can be found `None` is returned.
    ///
    /// If the catalog is `None` the original `SongId` is returned.
    pub fn prepare_song_id(
        &self,
        original_song_id: SongId,
        catalog: Option<&Catalog>,
    ) -> Option<SongId> {
        match catalog {
            None => Some(original_song_id),
            Some(catalog) => {
                if catalog.contains_id(original_song_id.clone()) {
                    return Some(original_song_id);
                }

                match percent_decode_str(original_song_id.as_str()).decode_utf8() {
                    Ok(decoded) => {
                        // let decoded = decoded.to_string();
                        if decoded != original_song_id.as_str() {
                            let string = decoded.to_string();
                            if catalog.contains_id(string.clone()) {
                                return Some(string.into());
                            }
                        }
                        None
                    }
                    Err(_) => None,
                }
            }
        }
    }
}
