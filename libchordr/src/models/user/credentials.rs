use crate::models::user::{Password, Username};
use serde::{Deserialize, Serialize};

/// Login credentials
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Credentials {
    username: Username,
    password: Password,
}

impl Credentials {
    pub fn new<U: Into<Username>, P: Into<Password>>(username: U, password: P) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}
