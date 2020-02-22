use crate::error::{Error, Result};
use crate::service::{ServiceConfigurationTrait, ServiceIdentifier};
use std::path::{Path, PathBuf};

/// A container for Service Configuration data read from user input or a configuration file
#[derive(Clone)]
pub struct AbstractServiceConfig {
    api_key: Result<String>,
    url: Result<String>,
    remote_directory: Result<String>,
    username: Result<String>,
    password: Result<String>,
    local_directory: PathBuf,
    identifier: ServiceIdentifier,
}

impl AbstractServiceConfig {
    pub fn build(
        api_key: Result<String>,
        url: Result<String>,
        remote_directory: Result<String>,
        username: Result<String>,
        password: Result<String>,
        local_directory: PathBuf,
        identifier: ServiceIdentifier,
    ) -> Self {
        Self {
            api_key,
            url,
            remote_directory,
            username,
            password,
            local_directory,
            identifier,
        }
    }

    pub fn api_key(&self) -> Result<String, Error> {
        self.api_key.clone()
    }

    pub fn url(&self) -> Result<String, Error> {
        self.url.clone()
    }

    pub fn remote_directory(&self) -> Result<String, Error> {
        self.remote_directory.clone()
    }

    pub fn username(&self) -> Result<String, Error> {
        self.username.clone()
    }

    pub fn password(&self) -> Result<String, Error> {
        self.password.clone()
    }
}

impl ServiceConfigurationTrait for AbstractServiceConfig {
    fn from_service_config(service_config: AbstractServiceConfig) -> Result<Self, Error>
        where
            Self: Sized,
    {
        Ok(service_config)
    }

    fn identifier(&self) -> ServiceIdentifier {
        self.identifier
    }

    fn local_directory(&self) -> &Path {
        self.local_directory.as_path()
    }
}
