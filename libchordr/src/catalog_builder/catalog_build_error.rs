use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct CatalogBuildError {
    message: String,
    path: PathBuf,
    inner: Option<Box<dyn Error>>,
}

impl CatalogBuildError {
    pub(super) fn new<S: Into<String>, P: Into<PathBuf>>(message: S, path: P) -> Self {
        Self {
            message: message.into(),
            path: path.into(),
            inner: None,
        }
    }
    pub(super) fn from_error<E: Error + 'static, P: Into<PathBuf>>(error: E, path: P) -> Self {
        Self {
            message: error.to_string(),
            path: path.into(),
            inner: Some(Box::new(error)),
        }
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Error for CatalogBuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.inner {
            None => None,
            Some(ref b) => Some(b.as_ref()),
        }
    }
}

impl Display for CatalogBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let description = match self.source() {
            Some(s) => s.to_string(),
            None => self.message.to_owned(),
        };

        write!(
            f,
            "Error during analysis of file {}: {}",
            self.path.to_string_lossy(),
            description
        )
    }
}
