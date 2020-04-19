use crate::errors::WebError;
use libchordr::models::setlist::Setlist;
use libchordr::prelude::{SetlistEntry, SongIdTrait};

pub struct SerializeService {}

impl SerializeService {
    /// Serialize `setlist` into a string
    pub fn serialize(setlist: &Setlist<SetlistEntry>) -> Result<String, WebError> {
        Ok(setlist
            .iter()
            .map(|song| song.id().to_string())
            .collect::<Vec<String>>()
            .join(","))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::entry;

    #[test]
    fn serialize_test() {
        let list = Setlist::with_entries(vec![
            entry("0"),
            entry("1"),
            entry("2"),
            entry("3"),
            entry("4"),
        ]);
        let result = SerializeService::serialize(&list);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "0,1,2,3,4");
    }
}
