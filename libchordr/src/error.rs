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
    inner: Kind,
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

    pub fn io_error(error: std::io::Error) -> Self {
        Self::new(Kind::Io(error))
    }

    pub fn notation_error(error: NotationError) -> Self {
        Self::new(Kind::Notation(error))
    }

    pub fn semitone_notation_error(error: SemitoneNotationError) -> Self {
        Self::new(Kind::SemitoneNotation(error))
    }

    pub fn parse_int_error(error: std::num::ParseIntError) -> Self {
        Self::new(Kind::ParseInt(error))
    }

    pub fn unknown_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::Unknown(description.into()))
    }

    fn new(kind: Kind) -> Self {
        Error { inner: kind }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.inner)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.inner)
    }
}

impl From<::std::io::Error> for Error {
    fn from(error: ::std::io::Error) -> Self {
        Self::new(Kind::Io(error))
    }
}

impl From<NotationError> for Error {
    fn from(error: NotationError) -> Self {
        Self::new(Kind::Notation(error))
    }
}

impl From<SemitoneNotationError> for Error {
    fn from(error: SemitoneNotationError) -> Self {
        Self::new(Kind::SemitoneNotation(error))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::new(Kind::ParseInt(error))
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
    Io(std::io::Error),
    Notation(NotationError),
    SemitoneNotation(SemitoneNotationError),
    ParseInt(std::num::ParseIntError),
}

impl StdError for Kind {}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Kind::Parser(s) => f.write_str(&s),
            Kind::TagBuilder(s) => f.write_str(&s),
            Kind::CatalogBuilderFatal(s, p) => {
                write!(f, "Error while building catalog: {} for path {:?}", s, p)
            }
            Kind::FileType(s) => f.write_str(&s),
            Kind::Chord(s) => f.write_str(&s),
            Kind::Setlist(s) => f.write_str(&s),
            Kind::Unknown(s) => f.write_str(&s),
            Kind::InvalidUsername(_name, message) => f.write_str(message),
            Kind::InvalidPassword(_password, message) => f.write_str(message),
            Kind::InvalidTeamId(invalid_id, _message) => {
                write!(f, "Team ID '{}' is not valid", invalid_id)
            }
            Kind::Io(i) => write!(f, "{}", i),
            Kind::Notation(i) => write!(f, "{}", i),
            Kind::SemitoneNotation(i) => write!(f, "{}", i),
            Kind::ParseInt(i) => write!(f, "{}", i),
        }
    }
}
