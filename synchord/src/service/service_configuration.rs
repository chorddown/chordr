use crate::error::Result;
// pub use crate::service::abstract_service_config::ServiceConfigTrait;
use crate::service::{AbstractServiceConfig, ServiceIdentifier};
use std::path::Path;

pub trait ServiceConfigurationTrait {
    /// Build a new instance of the Service Configuration from the values of the given [`AbstractServiceConfig`]
    fn from_service_config(service_config: AbstractServiceConfig) -> Result<Self>
    where
        Self: Sized;

    /// Return the identifier for the associated service type
    fn identifier(&self) -> ServiceIdentifier;

    /// Return the local directory
    fn local_directory(&self) -> &Path;
}
