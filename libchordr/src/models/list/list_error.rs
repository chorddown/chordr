use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ListError {
    AlreadyInList,
    NotFound,
    MoveError(String),
}

impl Display for ListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::AlreadyInList => f.write_str("Already in list"),
            Self::NotFound => f.write_str("Already in list"),
            Self::MoveError(s) => f.write_str(s),
        }
    }
}

impl Error for ListError {}
