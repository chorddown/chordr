use crate::error::Error;
use std::fmt;

use crate::helper::validate_model_identifier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TeamId(String);

impl TeamId {
    pub fn new<S: Into<String>>(id: S) -> Result<Self, Error> {
        let id = id.into();
        match validate_model_identifier(&id) {
            Ok(_) => Ok(Self(id)),
            Err(msg) => Err(Error::invalid_team_id_error(id, msg))
        }
    }
}

impl fmt::Display for TeamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}
