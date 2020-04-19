use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use stdweb::web::error::{IError, SecurityError};

#[derive(Debug)]
pub enum WebError {
    SortableError(String),
    JsError(String),
}

impl WebError {
    pub fn sortable_error<S: Into<String>>(s: S) -> Self {
        WebError::SortableError(s.into())
    }

    pub fn js_error<S: Into<String>>(s: S) -> Self {
        WebError::JsError(s.into())
    }
}

impl Display for WebError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                WebError::SortableError(s) => s,
                WebError::JsError(s) => s,
            }
        )
    }
}

impl Error for WebError {}

impl From<SecurityError> for WebError {
    fn from(e: SecurityError) -> Self {
        WebError::JsError(e.message())
    }
}
