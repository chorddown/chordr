use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct SerializerError {
    inner: Box<dyn Error + 'static>,
}

impl SerializerError {
    pub(crate) fn from_error<E: Error + 'static>(inner: E) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl Debug for SerializerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SerializerError: {:?}", self.inner)
    }
}

impl Display for SerializerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Serializing failed: {}", self.inner)
    }
}

impl Error for SerializerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.inner)
    }
}
