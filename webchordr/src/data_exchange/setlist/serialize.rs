use crate::errors::WebError;
use libchordr::models::list::ListEntryTrait;
use libchordr::models::setlist::Setlist;

pub struct SerializeService {}

impl SerializeService {
    /// Serialize `setlist` into a string
    pub fn serialize(setlist: &Setlist) -> Result<String, WebError> {
        Ok(setlist
            .clone()
            .into_iter()
            .map(|song| song.id().to_string())
            .collect::<Vec<String>>()
            .join(","))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::{entry, get_test_user};
    use chrono::Utc;

    #[test]
    fn serialize_test() {
        let list = Setlist::new(
            "Setlist name",
            1,
            get_test_user(),
            None,
            Some(Utc::now()),
            Utc::now(),
            Utc::now(),
            vec![entry("0"), entry("1"), entry("2"), entry("3"), entry("4")],
        );
        let result = SerializeService::serialize(&list);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "0,1,2,3,4");
    }
}
