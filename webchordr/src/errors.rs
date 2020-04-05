use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum WebError {
    SortableError(String)
}

impl WebError {
    pub fn sortable_error<S: Into<String>>(s: S) -> Self {
        WebError::SortableError(s.into())
    }
}

impl Display for WebError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            WebError::SortableError(s) => s
        })
    }
}

impl Error for WebError {}
