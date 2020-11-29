use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::path::PathBuf;

/// Shorthand for chord library results
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

/// Error type for errors raised in the chord library
#[derive(Debug)]
pub struct Error {
    inner: Box<dyn StdError>,
}

#[doc(hidden)]
impl Error {
    pub fn parser_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::ParserError(description.into()))
    }

    pub fn tag_builder_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::TagBuilderError(description.into()))
    }

    pub fn catalog_builder_fatal_error<S: Into<String>>(description: S, path: PathBuf) -> Self {
        Error::new(Kind::CatalogBuilderFatalError(description.into(), path))
    }

    pub fn file_type_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::FileTypeError(description.into()))
    }

    pub fn chord_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::ChordError(description.into()))
    }

    pub fn setlist_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::SetlistError(description.into()))
    }

    pub fn invalid_username_error<S1: Into<String>, S2: Into<String>>(
        invalid_username: S1,
        message: S2,
    ) -> Self {
        Error::new(Kind::InvalidUsernameError(
            invalid_username.into(),
            message.into(),
        ))
    }

    pub fn invalid_password_error<S1: Into<String>, S2: Into<String>>(
        invalid_password: S1,
        message: S2,
    ) -> Self {
        Error::new(Kind::InvalidPasswordError(
            invalid_password.into(),
            message.into(),
        ))
    }

    pub fn invalid_team_id_error<S1: Into<String>, S2: Into<String>>(
        invalid_team_id: S1,
        message: S2,
    ) -> Self {
        Error::new(Kind::InvalidTeamIdError(
            invalid_team_id.into(),
            message.into(),
        ))
    }

    pub fn unknown_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::UnknownError(description.into()))
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

#[derive(Debug)]
enum Kind {
    ParserError(String),
    TagBuilderError(String),
    CatalogBuilderFatalError(String, PathBuf),
    FileTypeError(String),
    ChordError(String),
    SetlistError(String),
    UnknownError(String),
    InvalidUsernameError(String, String),
    InvalidPasswordError(String, String),
    InvalidTeamIdError(String, String),
}

impl StdError for Kind {}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Kind::ParserError(s) => write!(f, "{}", s),
            Kind::TagBuilderError(s) => write!(f, "{}", s),
            Kind::CatalogBuilderFatalError(s, p) => {
                write!(f, "Error while building catalog: {} for path {:?}", s, p)
            }
            Kind::FileTypeError(s) => write!(f, "{}", s),
            Kind::ChordError(s) => write!(f, "{}", s),
            Kind::SetlistError(s) => write!(f, "{}", s),
            Kind::UnknownError(s) => write!(f, "{}", s),
            Kind::InvalidUsernameError(_name, message) => write!(f, "{}", message),
            Kind::InvalidPasswordError(_password, message) => write!(f, "{}", message),
            Kind::InvalidTeamIdError(invalid_id, _message) => {
                write!(f, "Team ID '{}' is not valid", invalid_id)
            }
        }
    }
}
