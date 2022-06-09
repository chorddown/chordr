use crate::data_exchange::serializer_error::SerializerError;
use crate::data_exchange::serializer_trait::DxSerializer;
use crate::data_exchange::setlist::SharingSetlistWithVersion;
use crate::data_exchange::VERSION;
use crate::models::setlist::sharing_setlist::SharingSetlist;
use crate::models::setlist::Setlist;

pub struct SerializeService {}

impl DxSerializer for SerializeService {
    type Subject = Setlist;

    fn serialize(value: &Self::Subject) -> Result<String, SerializerError> {
        let sharing_setlist = SharingSetlistWithVersion {
            version: VERSION.to_string(),
            setlist: SharingSetlist::from(value),
        };
        Ok(serde_qs::to_string(&sharing_setlist)?)
    }
}

impl From<serde_qs::Error> for SerializerError {
    fn from(e: serde_qs::Error) -> Self {
        SerializerError::from_error(e)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_exchange::setlist::test::{build_test_setlist, build_test_setlist_string};

    #[test]
    fn serialize_test() {
        let list = build_test_setlist();
        let result = SerializeService::serialize(&list);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), build_test_setlist_string());
    }
}
