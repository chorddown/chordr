pub mod deserializer_error;
pub mod deserializer_trait;
pub mod serde_helper;
pub mod serializer_error;
pub mod serializer_trait;
pub mod setlist;

pub use deserializer_error::DeserializerError;
pub use deserializer_trait::DxDeserializer;
pub use serializer_error::SerializerError;
pub use serializer_trait::DxSerializer;

const VERSION: &'static str = "2";
