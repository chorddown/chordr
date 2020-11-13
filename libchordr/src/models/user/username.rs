use crate::error::Error;
use crate::helper::validate_model_identifier;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Username(String);

impl Username {
    pub fn new<S: Into<String>>(id: S) -> Result<Self, Error> {
        let id = id.into();
        match validate_model_identifier(&id) {
            Ok(_) => Ok(Self(id)),
            Err(msg) => Err(Error::invalid_username_error(id, msg))
        }
    }
}

impl TryFrom<&str> for Username {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Username::new(value)
    }
}

impl TryFrom<String> for Username {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Username::new(value)
    }
}

impl TryFrom<&String> for Username {
    type Error = Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Username::new(value)
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
