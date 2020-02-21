use super::{RecurringTaskTrait, TaskTrait};
use crate::configuration::Configuration;
use crate::error::Result;
use libsynchord::error::Error as SynchordError;
use libsynchord::prelude::{ServiceConfig, Services};
use log::info;
use std::env;

pub struct DownloadTask {
    service_config: ServiceConfig,
    service: Services,
}

impl TaskTrait for DownloadTask {
    fn with_configuration(configuration: &Configuration) -> Result<Self> {
        let service_config = build_service_config(configuration);
        let service = get_service(configuration, &service_config)?;

        Ok(Self {
            service_config,
            service,
        })
    }
}

impl RecurringTaskTrait for DownloadTask {
    fn run(&self) -> Result<()> {
        info!("Run Download Task");
        libsynchord::helper::download(&self.service, &self.service_config)?;

        Ok(())
    }
}

fn get_service(configuration: &Configuration, service_config: &ServiceConfig) -> Result<Services> {
    Ok(Services::build_service_by_identifier(
        &configuration.service.identifier.to_string(),
        service_config,
    )?)
}

fn build_service_config(configuration: &Configuration) -> ServiceConfig {
    let api_token = if !(configuration.service.api_token.trim().is_empty()) {
        Ok(configuration.service.api_token.trim().to_owned())
    } else {
        get_api_key()
    };

    let password = if !(configuration.service.password.trim().is_empty()) {
        Ok(configuration.service.password.trim().to_owned())
    } else {
        get_password()
    };
    ServiceConfig::new(
        api_token,
        Ok(configuration.service.url.clone()),
        Ok(configuration.service.remote_directory.clone()),
        Ok(configuration.service.username.clone()),
        password,
        Ok(configuration.output_directory.clone()),
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
