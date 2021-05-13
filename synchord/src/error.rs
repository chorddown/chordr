use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter};

use dropbox_sdk::files::{DownloadError, ListFolderError};
use reqwest::Error as RequestError;
use xml::reader::Error as XmlError;

/// Shorthand for synchord results
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

/// Error type for errors raised in synchord
#[derive(Debug, Clone)]
pub struct Error {
    inner: Kind,
}

#[doc(hidden)]
impl Error {
    pub fn download_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::DownloadError(description.into()))
    }

    pub fn skip_download<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::SkipDownload(description.into()))
    }

    pub fn missing_argument_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::MissingArgumentError(description.into()))
    }

    pub fn invalid_argument_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::InvalidArgumentError(description.into()))
    }

    pub fn unknown_service_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::UnknownServiceError(description.into()))
    }

    pub fn io_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::IoError(description.into()))
    }

    pub fn xml_parser_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::XMLParserError(description.into()))
    }

    pub fn url_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::UrlError(description.into()))
    }

    fn new(kind: Kind) -> Self {
        Error { inner: kind }
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
        Error::io_error(format!("{}", error))
    }
}

impl From<dropbox_sdk::Error> for Error {
    fn from(error: dropbox_sdk::Error) -> Self {
        Error::download_error(format!("{}", error))
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

impl From<RequestError> for Error {
    fn from(error: RequestError) -> Self {
        Error::download_error(format!("{}", error))
    }
}

impl From<XmlError> for Error {
    fn from(error: XmlError) -> Self {
        Error::xml_parser_error(format!("{}", error))
    }
}

impl From<::chrono::format::ParseError> for Error {
    fn from(error: ::chrono::format::ParseError) -> Self {
        Error::xml_parser_error(format!("{}", error))
    }
}

// impl From<::url::ParseError> for Error {
//     fn from(error: ::url::ParseError) -> Self {
//         Error::url_error(format!("{}", error))
//     }
// }

//impl From<&Error> for Error {
//    fn from(error: &Error) -> Self {
//        Error::from_error(Kind::UnknownError(format!("{}", error)))
//    }
//}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Kind {
    /// Error trying to download files or file information
    DownloadError(String),

    /// "Error" kind signaling why a download was skipped
    SkipDownload(String),

    /// Error if an unknown service was requested
    UnknownServiceError(String),

    /// Error if a required argument is missing
    MissingArgumentError(String),

    /// Error if an argument is invalid
    InvalidArgumentError(String),

    /// Error during file IO
    IoError(String),

    /// Error during XML parsing
    XMLParserError(String),

    /// Error during URL parsing
    UrlError(String),

    /// Unknown/uncategorized error
    UnknownError(String),
}

impl StdError for Kind {}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Kind::DownloadError(s) => write!(f, "Download error: {}", s),
            Kind::SkipDownload(s) => write!(f, "{}", s),
            Kind::UnknownServiceError(s) => write!(f, "Unknown service error: {}", s),
            Kind::MissingArgumentError(s) => write!(f, "Missing argument error: {}", s),
            Kind::InvalidArgumentError(s) => write!(f, "Invalid argument error: {}", s),
            Kind::IoError(s) => write!(f, "IO error: {}", s),
            Kind::XMLParserError(s) => write!(f, "XML parser error: {}", s),
            Kind::UrlError(s) => write!(f, "URL error: {}", s),
            Kind::UnknownError(s) => write!(f, "Unknown error: {}", s),
        }
    }
}
