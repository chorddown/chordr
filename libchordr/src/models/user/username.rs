use crate::error::Error;
use crate::helper::is_valid_model_identifier;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Username(String);

impl Username {
    pub fn new<S: Into<String>>(id: S) -> Result<Self, Error> {
        let id = id.into();
        if is_valid_model_identifier(&id) {
            Ok(Self(id))
        } else {
            Err(Error::invalid_username_error(id))
        }
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}
