use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SrvError {
    inner: Box<dyn Error>,
}

impl SrvError {
    pub fn persistence_error<S: Into<String>>(msg: S) -> Self {
        Self::from_kind(SrvErrorKind::PersistenceError(msg.into()))
    }

    fn from_kind(error: SrvErrorKind) -> Self {
        Self {
            inner: Box::new(error)
        }
    }

    fn from_error<E: Error + 'static>(error: E) -> Self {
        Self {
            inner: Box::new(error),
        }
    }
}

impl fmt::Display for SrvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Error for SrvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.inner.as_ref())
    }
}

impl From<::diesel::result::Error> for SrvError {
    fn from(error: ::diesel::result::Error) -> Self {
        // SrvError::persistence_error(error.to_string())
        SrvError::from_error(error)
    }
}

#[derive(Debug)]
pub enum SrvErrorKind {
    PersistenceError(String)
}

impl fmt::Display for SrvErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SrvErrorKind::PersistenceError(s) => write!(f, "{}", s),
        }
    }
}

impl Error for SrvErrorKind {}
