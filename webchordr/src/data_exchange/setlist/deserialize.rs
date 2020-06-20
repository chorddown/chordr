use crate::errors::WebError;
use chrono::Utc;
use libchordr::models::setlist::Setlist;
use libchordr::models::user::{User, Username};
use libchordr::prelude::{CatalogTrait, SetlistEntry, SongData};

pub struct DeserializeResult {
    pub setlist: Setlist,
    pub errors: Vec<WebError>,
}

pub struct DeserializeService {}

#[allow(unused)]
impl DeserializeService {
    /// Deserialize the given serialized `Setlist` by querying `catalog` for the `Song`s
    pub fn deserialize<E: SongData, C: CatalogTrait<E>>(
        serialized_setlist: &str,
        catalog: &C,
    ) -> DeserializeResult {
        unimplemented!();
        let (entries, errors) = Self::collect_setlist_entries(serialized_setlist, catalog);
        let now = Utc::now();

        DeserializeResult {
            setlist: Setlist::new(
                "missing-setlist-name",
                0,
                User::new(Username::new("unknown").unwrap(), "John", "Doe", ""),
                None,
                now,
                now,
                now,
                entries,
            ),
            errors,
        }
    }

    #[allow(unused)]
    fn collect_setlist_entries<E: SongData, C: CatalogTrait<E>>(
        serialized_setlist: &str,
        catalog: &C,
    ) -> (Vec<SetlistEntry>, Vec<WebError>) {
        let (entries, errors): (Vec<_>, Vec<_>) = serialized_setlist
            .split(',')
            .map(|song_id| match catalog.get(song_id) {
                Some(song) => Ok(SetlistEntry::from_song(song)),
                None => Err(WebError::setlist_deserialize_error(format!(
                    "Could not find song with ID '{}'",
                    song_id
                ))),
            })
            .partition(Result::is_ok);

        (
            entries.into_iter().map(Result::unwrap).collect(),
            errors.into_iter().map(Result::unwrap_err).collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::{entry, test_song, TestSong};
    use libchordr::models::song_id::SongId;
    use libchordr::prelude::ListEntryTrait;
    use std::slice::Iter;

    struct TestCatalog {
        pub songs: Vec<TestSong>,
    }

    impl CatalogTrait<TestSong> for TestCatalog {
        fn get<S: Into<SongId>>(&self, song_id: S) -> Option<&TestSong> {
            let song_id = song_id.into();
            self.songs.iter().find(|s| s.id() == song_id)
        }

        fn len(&self) -> usize {
            unreachable!()
        }

        fn iter(&self) -> Iter<TestSong> {
            unreachable!()
        }

        fn revision(&self) -> String {
            unreachable!()
        }
    }

    #[test]
    fn deserialize_test() {
        // TODO: Implementation
        return;
        let songs = vec![
            test_song("0"),
            test_song("1"),
            test_song("2"),
            test_song("3"),
            test_song("4"),
        ];
        let result = DeserializeService::deserialize("0,1,2,3,4", &TestCatalog { songs });
        let entries = vec![entry("0"), entry("1"), entry("2"), entry("3"), entry("4")];
        assert_eq!(result.setlist.name(), "missing-setlist-name");
        assert_eq!(
            result
                .setlist
                .into_iter()
                .collect::<Vec<SetlistEntry>>(),
            entries
        );
        assert!(result.errors.is_empty(),);
    }

    #[test]
    fn deserialize_w_error_test() {
        // TODO: Implementation
        return;
        let songs = vec![
            test_song("0"),
            test_song("1"),
            test_song("2"),
            test_song("3"),
            test_song("4"),
        ];
        let result = DeserializeService::deserialize("0,1,2,not-found,3,4", &TestCatalog { songs });
        let entries = vec![entry("0"), entry("1"), entry("2"), entry("3"), entry("4")];
        assert_eq!(result.setlist.name(), "missing-setlist-name");
        assert_eq!(
            result
                .setlist
                .into_iter()
                .collect::<Vec<SetlistEntry>>(),
            entries
        );
        assert!(!result.errors.is_empty(),);
        assert_eq!(
            result.errors[0].to_string(),
            "Could not find song with ID 'not-found'"
        );
    }
}
