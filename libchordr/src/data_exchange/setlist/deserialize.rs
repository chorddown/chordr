use crate::data_exchange::deserializer_error::DeserializerError;
use crate::data_exchange::deserializer_trait::DxDeserializer;
use crate::data_exchange::setlist::SharingSetlistWithVersion;
use crate::data_exchange::VERSION;
use crate::models::setlist::sharing_setlist::SharingSetlist;

pub struct DeserializeService {}

impl DxDeserializer for DeserializeService {
    type Subject = SharingSetlist;

    fn deserialize(value: &str) -> Result<Self::Subject, DeserializerError> {
        let result: SharingSetlistWithVersion = serde_qs::from_str(value)?;
        if result.version != VERSION {
            Err(DeserializerError::UnsupportedVersion(result.version))
        } else {
            Ok(result.setlist)
        }
    }
}

impl From<serde_qs::Error> for DeserializerError {
    fn from(e: serde_qs::Error) -> Self {
        DeserializerError::from_error(e)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_exchange::setlist::test::{build_test_setlist, build_test_setlist_string};

    #[test]
    fn deserialize_test() {
        let result = DeserializeService::deserialize(&build_test_setlist_string());
        assert!(result.is_ok(), "{}", result.unwrap_err().to_string());
        assert_eq!(result.unwrap(), SharingSetlist::from(build_test_setlist()));
    }
}
