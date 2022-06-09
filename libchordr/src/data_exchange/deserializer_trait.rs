use crate::data_exchange::deserializer_error::DeserializerError;

pub trait DxDeserializer {
    type Subject;
    fn deserialize(value: &str) -> Result<Self::Subject, DeserializerError>;
}
