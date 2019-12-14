use serde::{Serialize, Serializer};
use super::FileType;

impl Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}
