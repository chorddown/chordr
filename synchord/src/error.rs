use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter};
use dropbox_sdk::files::{ListFolderError, DownloadError};

/// Shorthand for synchord results
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

/// Error type for errors raised in the chord library
#[derive(Debug)]
pub struct Error {
    inner: Box<dyn StdError>,
}

#[doc(hidden)]
impl Error {
    pub fn download_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::DownloadError(description.into()))
    }

    pub fn missing_argument_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::MissingArgumentError(description.into()))
    }

    pub fn unknown_service_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::UnknownServiceError(description.into()))
    }

    pub fn io_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::IoError(description.into()))
    }

    pub fn from_error<E: StdError + 'static>(error: E) -> Self {
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

impl From<dropbox_sdk::Error> for Error {
    fn from(error: dropbox_sdk::Error) -> Self {
        Error::from_error(error)
    }
}

impl From<ListFolderError> for Error {
    fn from(error: ListFolderError) -> Self {
        Error::download_error(format!("{}", error))
    }
}

impl From<DownloadError> for Error {
    fn from(error: DownloadError) -> Self {
        Error::download_error(format!("{}", error))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Kind {
    DownloadError(String),
    UnknownServiceError(String),
    MissingArgumentError(String),
    IoError(String),
    UnknownError(String),
}

impl StdError for Kind {}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Kind::DownloadError(s) => write!(f, "Download error: {}", s),
            Kind::UnknownServiceError(s) => write!(f, "Unknown service error: {}", s),
            Kind::MissingArgumentError(s) => write!(f, "Missing argument error: {}", s),
            Kind::IoError(s) => write!(f, "IO error: {}", s),
            Kind::UnknownError(s) => write!(f, "Unknown error: {}", s),
        }
    }
}
