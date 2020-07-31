use super::{RecurringTaskTrait, TaskTrait};
use crate::configuration::Configuration;
use crate::error::Result;
use libsynchord::error::Error as SynchordError;
use libsynchord::prelude::{
    AbstractServiceConfig, ServiceConfigurationTrait, ServiceTrait, Services,
};
use log::info;
use std::env;

pub struct DownloadTask {
    service_config: AbstractServiceConfig,
    service: Services,
}

impl TaskTrait for DownloadTask {
    fn with_configuration(configuration: Configuration) -> Result<Self> {
        let service_config = build_service_config(configuration);
        let service = get_service(service_config.clone())?;

        Ok(Self {
            service_config,
            service,
        })
    }
}

impl RecurringTaskTrait for DownloadTask {
    fn run(&self) -> Result<()> {
        info!(
            "Run Download Task: Download files using service {} to {}",
            self.service.identifier(),
            self.service_config.local_directory().display()
        );
        libsynchord::helper::download(&self.service, &self.service_config)?;

        Ok(())
    }
}

fn get_service(service_config: AbstractServiceConfig) -> Result<Services> {
    Ok(Services::new(service_config)?)
}

fn build_service_config(configuration: Configuration) -> AbstractServiceConfig {
    let api_token = match &configuration.service.api_token {
        Some(v) if !v.trim().is_empty() => Ok(v.to_owned()),
        Some(_) => get_api_key(),
        None => get_api_key(),
    };

    let password = match &configuration.service.password {
        Some(v) if !v.trim().is_empty() => Ok(v.to_owned()),
        Some(_) => get_password(),
        None => get_password(),
    };

    AbstractServiceConfig::build(
        api_token,
        configuration
            .service
            .url
            .ok_or(SynchordError::missing_argument_error("URL")),
        configuration
            .service
            .remote_directory
            .ok_or(SynchordError::missing_argument_error("Remote-directory")),
        configuration
            .service
            .username
            .ok_or(SynchordError::missing_argument_error("Username")),
        password,
        configuration.output_directory.clone(),
        configuration.service.identifier,
    )
}

fn get_api_key() -> Result<String, SynchordError> {
    match env::var("API_TOKEN") {
        Ok(val) => Ok(val),
        Err(_) => Err(SynchordError::missing_argument_error(
            "No API token provided",
        )),
    }
}

fn get_password() -> Result<String, SynchordError> {
    match env::var("PASSWORD") {
        Ok(val) => Ok(val),
        Err(_) => Err(SynchordError::missing_argument_error(
            "No password provided",
        )),
    }
}
