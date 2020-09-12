use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
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

impl TryFrom<&str> for Password {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Password::new(value)
    }
}

impl TryFrom<String> for Password {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Password::new(value)
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for Password {
    fn default() -> Self {
        Self("******".to_string())
    }
}
