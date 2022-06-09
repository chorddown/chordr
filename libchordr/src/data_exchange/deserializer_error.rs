use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum DeserializerError {
    BubbledError(Box<dyn Error + 'static>),
    UnsupportedVersion(String),
}

impl DeserializerError {
    pub(crate) fn from_error<E: Error + 'static>(inner: E) -> Self {
        Self::BubbledError(Box::new(inner))
    }
}

impl Debug for DeserializerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for DeserializerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializerError::BubbledError(i) => {
                write!(f, "Deserializing failed: {:?}", i)
            }
            DeserializerError::UnsupportedVersion(v) => {
                write!(f, "Deserializing failed: Unsupported version '{:?}'", v)
            }
        }
    }
}
