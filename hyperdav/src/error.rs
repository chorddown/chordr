use reqwest::Error as ReqwestError;
use reqwest::StatusCode;

use crate::response::PropfindParseError;

/// Result which uses failure::Error by default.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Our custom error type using Failure.
#[derive(Debug)]
pub enum Error {
    /// Used when a networking error occurs.
    NetworkingError(::reqwest::Error),
    /// Used when propfind fails.
    PropfindParse(PropfindParseError),
    /// Used when the request failes.
    FailedRequest(StatusCode),
    /// Used when we cannot parse the URL.
    UrlParsingError(ReqwestError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NetworkingError(e) => write!(f, "{:?}", e),
            Error::PropfindParse(e) => write!(f, "Failed to propfind: {}", e),
            Error::FailedRequest(e) => write!(f, "Request failed, error code: {}", e),
            Error::UrlParsingError(e) => write!(f, "Parsing URL failed: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::UrlParsingError(value)
    }
}

impl From<PropfindParseError> for Error {
    fn from(value: PropfindParseError) -> Self {
        Error::PropfindParse(value)
    }
}
