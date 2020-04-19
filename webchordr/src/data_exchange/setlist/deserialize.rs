use crate::errors::WebError;
use libchordr::models::setlist::Setlist;
use libchordr::prelude::{Catalog, SetlistEntry};

pub struct DeserializeResult {
    pub setlist: Setlist<SetlistEntry>,
    pub errors: Vec<WebError>,
}

pub struct DeserializeService {}

impl DeserializeService {
    /// Deserialize the given serialized `Setlist` by querying `catalog` for the `Song`s
    pub fn deserialize(
        serialized_setlist: &str,
        catalog: &Catalog,
    ) -> DeserializeResult {
        let (entries, errors) = Self::collect_setlist_entries(serialized_setlist, catalog);

        DeserializeResult {
            setlist: Setlist::with_entries(entries),
            errors,
        }
    }

    fn collect_setlist_entries(
        serialized_setlist: &str,
        catalog: &Catalog,
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
    use crate::test_helpers::entry;

    #[test]
    fn deserialize_test() {
        // let mut list = Setlist::with_entries(vec![entry("0"), entry("1"), entry("2"), entry("3"), entry("4")]);
        // assert_eq!(SetlistSerializerService::serialize(&list), "0,1,2,3,4");
    }
}
