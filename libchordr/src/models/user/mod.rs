mod username;

pub use self::username::Username;
use serde::{Deserialize, Serialize};

/// User of the application which has it's own `Setlist`s and login credentials
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    username: Username,
    first_name: String,
    last_name: String,
    password: String,
}

impl User {
    pub fn new<U: Into<Username>, F: Into<String>, L: Into<String>, P: Into<String>>(
        username: U,
        first_name: F,
        last_name: L,
        password: P,
    ) -> Self {
        Self {
            username: username.into(),
            first_name: first_name.into(),
            last_name: last_name.into(),
            password: password.into(),
        }
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
