use crate::errors::SharingError;
use crate::errors::WebError;
use libchordr::data_exchange::setlist::{
    DeserializeService as LibChordrDeserializeService, SharingSetlist,
};
use libchordr::data_exchange::DxDeserializer;
use libchordr::models::setlist::sharing_setlist_entry::SharingSetlistEntry;
use libchordr::models::setlist::Setlist;
use libchordr::models::user::User;
use libchordr::prelude::{CatalogTrait, Formatting, SetlistEntry, SongData, SongSettings};
use percent_encoding::percent_decode_str;

pub struct DeserializeResult {
    pub setlist: Setlist,
    pub errors: Vec<WebError>,
}

pub struct DeserializeService {}

#[allow(unused, unreachable_code)]
impl DeserializeService {
    /// Deserialize the given serialized `Setlist` by querying `catalog` for the `Song`s
    pub fn deserialize<E: SongData, C: CatalogTrait<E>>(
        serialized_setlist: &str,
        catalog: &C,
    ) -> Result<DeserializeResult, WebError> {
        // Closure that tries to percent-decode the input string and deserialize it afterwards
        let try_percent_decoded =
            |initial_error| match percent_decode_str(serialized_setlist).decode_utf8() {
                Ok(decoded) => LibChordrDeserializeService::deserialize(decoded.as_ref()),
                // We ignore the percent-decode-error and return the initial deserialize-error
                Err(_) => Err(initial_error),
            };

        let sharing_setlist = LibChordrDeserializeService::deserialize(serialized_setlist)
            .or_else(try_percent_decoded)
            .map_err(SharingError::from)?;

        let (entries, errors) = Self::collect_setlist_entries(&sharing_setlist, catalog);

        let SharingSetlist {
            name,
            id,
            songs,
            gig_date,
            creation_date,
            modification_date,
        } = sharing_setlist;

        Ok(DeserializeResult {
            setlist: Setlist::new(
                name,
                id,
                User::unknown(),
                None,
                gig_date,
                creation_date,
                modification_date,
                entries,
            ),
            errors,
        })
    }

    fn collect_setlist_entries<E: SongData, C: CatalogTrait<E>>(
        sharing_setlist: &SharingSetlist,
        catalog: &C,
    ) -> (Vec<SetlistEntry>, Vec<WebError>) {
        let (entries, errors): (Vec<_>, Vec<_>) = sharing_setlist
            .songs
            .iter()
            .map(|entry: &SharingSetlistEntry| Self::build_entry(catalog, entry))
            .partition(Result::is_ok);

        (
            entries.into_iter().map(Result::unwrap).collect(),
            errors.into_iter().map(Result::unwrap_err).collect(),
        )
    }

    fn build_entry<E: SongData, C: CatalogTrait<E>>(
        catalog: &C,
        entry: &SharingSetlistEntry,
    ) -> Result<SetlistEntry, WebError> {
        match catalog.get(&entry.id) {
            Some(song) => {
                let settings = SongSettings::new(
                    entry.transpose_semitone.unwrap_or_default(),
                    Formatting::default(),
                    entry.note.clone().unwrap_or_default(),
                );
                Ok(SetlistEntry::from_song_with_settings(song, settings))
            }
            None => Err(WebError::sharing_error(SharingError::SongNotFound(
                format!("Could not find song with ID '{}'", entry.id),
            ))),
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::test_helpers::{entry, test_song, TestSong};
//     use libchordr::models::song_id::SongId;
//     use libchordr::prelude::ListEntryTrait;
//     use std::slice::Iter;
//
//     struct TestCatalog {
//         pub songs: Vec<TestSong>,
//     }
//
//     impl CatalogTrait<TestSong> for TestCatalog {
//         fn get<S: Into<SongId>>(&self, song_id: S) -> Option<&TestSong> {
//             let song_id = song_id.into();
//             self.songs.iter().find(|s| s.id() == song_id)
//         }
//
//         fn len(&self) -> usize {
//             unreachable!()
//         }
//
//         fn is_empty(&self) -> bool {
//             self.songs.is_empty()
//         }
//
//         fn iter(&self) -> Iter<TestSong> {
//             unreachable!()
//         }
//
//         fn revision(&self) -> String {
//             unreachable!()
//         }
//     }
//
//     #[test]
//     #[allow(unreachable_code)]
//     fn deserialize_test() {
//         // TODO: Implementation
//         return;
//         let songs = vec![
//             test_song("0"),
//             test_song("1"),
//             test_song("2"),
//             test_song("3"),
//             test_song("4"),
//         ];
//         let result = DeserializeService::deserialize("0,1,2,3,4", &TestCatalog { songs });
//         let entries = vec![entry("0"), entry("1"), entry("2"), entry("3"), entry("4")];
//         assert_eq!(result.setlist.name(), "missing-setlist-name");
//         assert_eq!(
//             result.setlist.into_iter().collect::<Vec<SetlistEntry>>(),
//             entries
//         );
//         assert!(result.errors.is_empty(),);
//     }
//
//     #[test]
//     #[allow(unreachable_code)]
//     fn deserialize_w_error_test() {
//         // TODO: Implementation
//         return;
//         let songs = vec![
//             test_song("0"),
//             test_song("1"),
//             test_song("2"),
//             test_song("3"),
//             test_song("4"),
//         ];
//         let result = DeserializeService::deserialize("0,1,2,not-found,3,4", &TestCatalog { songs });
//         let entries = vec![entry("0"), entry("1"), entry("2"), entry("3"), entry("4")];
//         assert_eq!(result.setlist.name(), "missing-setlist-name");
//         assert_eq!(
//             result.setlist.into_iter().collect::<Vec<SetlistEntry>>(),
//             entries
//         );
//         assert!(!result.errors.is_empty(),);
//         assert_eq!(
//             result.errors[0].to_string(),
//             "Could not find song with ID 'not-found'"
//         );
//     }
// }
