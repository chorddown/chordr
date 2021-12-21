use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::path::PathBuf;

use crate::models::meta::b_notation::NotationError;
use crate::models::meta::semitone_notation::SemitoneNotationError;

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
        Error::new(Kind::Parser(description.into()))
    }

    pub fn tag_builder_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::TagBuilder(description.into()))
    }

    pub fn catalog_builder_fatal_error<S: Into<String>>(description: S, path: PathBuf) -> Self {
        Error::new(Kind::CatalogBuilderFatal(description.into(), path))
    }

    pub fn file_type_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::FileType(description.into()))
    }

    pub fn chord_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::Chord(description.into()))
    }

    pub fn setlist_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::Setlist(description.into()))
    }

    pub fn invalid_username_error<S1: Into<String>, S2: Into<String>>(
        invalid_username: S1,
        message: S2,
    ) -> Self {
        Error::new(Kind::InvalidUsername(
            invalid_username.into(),
            message.into(),
        ))
    }

    pub fn invalid_password_error<S1: Into<String>, S2: Into<String>>(
        invalid_password: S1,
        message: S2,
    ) -> Self {
        Error::new(Kind::InvalidPassword(
            invalid_password.into(),
            message.into(),
        ))
    }

    pub fn invalid_team_id_error<S1: Into<String>, S2: Into<String>>(
        invalid_team_id: S1,
        message: S2,
    ) -> Self {
        Error::new(Kind::InvalidTeamId(invalid_team_id.into(), message.into()))
    }

    pub fn unknown_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::Unknown(description.into()))
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
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

impl From<NotationError> for Error {
    fn from(error: NotationError) -> Self {
        Error::from_error(error)
    }
}

impl From<SemitoneNotationError> for Error {
    fn from(error: SemitoneNotationError) -> Self {
        Error::from_error(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::from_error(error)
    }
}

#[derive(Debug)]
enum Kind {
    Parser(String),
    TagBuilder(String),
    CatalogBuilderFatal(String, PathBuf),
    FileType(String),
    Chord(String),
    Setlist(String),
    Unknown(String),
    InvalidUsername(String, String),
    InvalidPassword(String, String),
    InvalidTeamId(String, String),
}

impl StdError for Kind {}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Kind::Parser(s) => write!(f, "{}", s),
            Kind::TagBuilder(s) => write!(f, "{}", s),
            Kind::CatalogBuilderFatal(s, p) => {
                write!(f, "Error while building catalog: {} for path {:?}", s, p)
            }
            Kind::FileType(s) => write!(f, "{}", s),
            Kind::Chord(s) => write!(f, "{}", s),
            Kind::Setlist(s) => write!(f, "{}", s),
            Kind::Unknown(s) => write!(f, "{}", s),
            Kind::InvalidUsername(_name, message) => write!(f, "{}", message),
            Kind::InvalidPassword(_password, message) => write!(f, "{}", message),
            Kind::InvalidTeamId(invalid_id, _message) => {
                write!(f, "Team ID '{}' is not valid", invalid_id)
            }
        }
    }
}
