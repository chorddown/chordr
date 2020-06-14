use crate::errors::WebError;
use libchordr::models::setlist::Setlist;
use libchordr::prelude::{SongIdTrait, SongListTrait};

pub struct SerializeService {}

impl SerializeService {
    /// Serialize `setlist` into a string
    pub fn serialize(setlist: &Setlist) -> Result<String, WebError> {
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
    use chrono::Utc;

    #[test]
    fn serialize_test() {
        let list = Setlist::new(
            "Setlist name",
            1,
            Utc::now(),
            Utc::now(),
            Utc::now(),
            vec![entry("0"), entry("1"), entry("2"), entry("3"), entry("4")],
        );
        let result = SerializeService::serialize(&list);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "0,1,2,3,4");
    }
}
