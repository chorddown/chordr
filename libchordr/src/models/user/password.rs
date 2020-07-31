use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn new<S: Into<String>>(password: S) -> Result<Self, Error> {
        let password = password.into();
        if password.is_empty() || password.len() < 8 {
            Err(Error::invalid_password_error(password))
        } else {
            Ok(Self(password))
        }
    }

    pub fn hashed<S: Into<String>>(hash: S) -> Result<Self, Error> {
        let hash = hash.into();
        // if hash.is_empty() || hash.len() < 8 {
        //     warn!("Hash is empty or too short");
        // }
        Ok(Self(hash))
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}
