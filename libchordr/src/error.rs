use std::error::Error as StdError;
use std::fmt::{Formatter, Display, Error as FmtError};

/// Shorthand for chord library results
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

/// Error type for errors raised in the chord library
#[derive(Debug)]
pub struct Error {
    inner: Box<dyn StdError>
}

#[doc(hidden)]
impl Error {
    pub fn parser_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::ParserError(description.into()))
    }

    pub fn from_error<E: StdError + 'static>(error: E) -> Self {
        Error { inner: Box::new(error) }
    }

    fn new(kind: Kind) -> Self {
        Error { inner: Box::new(kind) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.inner)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner.as_ref())
    }
}

impl From<::std::io::Error> for Error {
    fn from(error: ::std::io::Error) -> Self {
        Error::from_error(error)
    }
}

#[derive(Debug)]
enum Kind {
    ParserError(String),
}

impl StdError for Kind {}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Kind::ParserError(s) => write!(f, "Parser error: {}", s),
        }
    }
}
