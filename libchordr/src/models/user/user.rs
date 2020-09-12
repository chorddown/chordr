use crate::models::user::{Password, Username};
use serde::{Deserialize, Serialize};

/// User of the application which has it's own `Setlist`s and login credentials
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    username: Username,
    first_name: String,
    last_name: String,

    // #[serde(skip_serializing)]
    #[serde(skip)]
    password: Password,
}

impl User {
    pub fn new<U: Into<Username>, F: Into<String>, L: Into<String>, P: Into<Password>>(
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

    pub fn unknown() -> Self {
        Self {
            username: Username::new("unknown").unwrap(),
            first_name: "John".to_owned(),
            last_name: "Doe".to_owned(),
            password: Password::new("undefined-password").unwrap(),
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

    pub fn password(&self) -> &Password {
        &self.password
    }
}
