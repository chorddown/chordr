use crate::data_exchange::serializer_error::SerializerError;

pub trait DxSerializer {
    type Subject;
    fn serialize(value: &Self::Subject) -> Result<String, SerializerError>;
}
