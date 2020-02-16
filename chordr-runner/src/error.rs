use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter};

/// Shorthand for synchord results
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

/// Error type for errors raised in synchord
#[derive(Debug)]
pub struct Error {
    inner: Box<dyn StdError>,
}

#[doc(hidden)]
impl Error {
    pub fn serialization_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::SerializationError(description.into()))
    }

    pub fn io_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::IoError(description.into()))
    }

    pub fn configuration_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::ConfigurationError(description.into()))
    }

    pub fn configuration_reader_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::ConfigurationReaderError(description.into()))
    }

    fn from_error<E: StdError + 'static>(error: E) -> Self {
        Error {
            inner: Box::new(error),
        }
    }

    fn new(kind: Kind) -> Self {
        Error {
            inner: Box::new(kind),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.inner)
    }
}

impl StdError for Error {}

impl From<::std::io::Error> for Error {
    fn from(error: ::std::io::Error) -> Self {
        Error::from_error(error)
    }
}

impl From<libchordr::prelude::Error> for Error {
    fn from(error: libchordr::prelude::Error) -> Self {
        Error::from_error(error)
    }
}

impl From<libsynchord::prelude::Error> for Error {
    fn from(error: libsynchord::prelude::Error) -> Self {
        Error::from_error(error)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Kind {
    /// Error during serialization
    SerializationError(String),

    /// Error during file IO
    IoError(String),

    /// Error with the configuration
    ConfigurationError(String),

    /// Error while reading the configuration
    ConfigurationReaderError(String),

    /// Unknown/uncategorized error
    UnknownError(String),
}

impl StdError for Kind {}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Kind::SerializationError(s) => write!(f, "Serialization error: {}", s),
            Kind::IoError(s) => write!(f, "IO error: {}", s),
            Kind::ConfigurationError(s) => write!(f, "Configuration error: {}", s),
            Kind::ConfigurationReaderError(s) => write!(f, "Configuration reader error: {}", s),
            Kind::UnknownError(s) => write!(f, "Unknown error: {}", s),
        }
    }
}
