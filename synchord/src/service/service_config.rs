use crate::error::{Error, Result};
use std::path::PathBuf;

pub trait ServiceConfigTrait {
    fn api_key(&self) -> Result<String, Error>;
    fn url(&self) -> Result<String, Error>;
    fn remote_directory(&self) -> Result<String, Error>;
    fn username(&self) -> Result<String, Error>;
    fn password(&self) -> Result<String, Error>;
    fn local_directory(&self) -> Result<PathBuf, Error>;
}

pub struct ServiceConfig {
    api_key: Result<String>,
    url: Result<String>,
    remote_directory: Result<String>,
    username: Result<String>,
    password: Result<String>,
    local_directory: Result<PathBuf>,
}

impl ServiceConfig {
    pub fn new(
        api_key: Result<String>,
        url: Result<String>,
        remote_directory: Result<String>,
        username: Result<String>,
        password: Result<String>,
        local_directory: Result<PathBuf>,
    ) -> Self {
        Self {
            api_key,
            url,
            remote_directory,
            username,
            password,
            local_directory,
        }
    }
}

impl ServiceConfigTrait for ServiceConfig {
    fn api_key(&self) -> Result<String, Error> {
        self.api_key.clone()
    }

    fn url(&self) -> Result<String, Error> {
        self.url.clone()
    }

    fn remote_directory(&self) -> Result<String, Error> {
        self.remote_directory.clone()
    }

    fn username(&self) -> Result<String, Error> {
        self.username.clone()
    }

    fn password(&self) -> Result<String, Error> {
        self.password.clone()
    }

    fn local_directory(&self) -> Result<PathBuf, Error> {
        self.local_directory.clone()
    }
}
