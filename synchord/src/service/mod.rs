mod abstract_service_config;
mod dropbox_service;
mod file_entry;
mod service_configuration;
mod web_dav_service;

pub use self::abstract_service_config::AbstractServiceConfig;
// pub use self::abstract_service_config::ServiceConfigTrait;
pub use self::dropbox_service::DropboxService;
pub use self::file_entry::FileEntry;
pub use self::service_configuration::ServiceConfigurationTrait;
pub use self::service_identifier::ServiceIdentifier;
pub use self::web_dav_service::WebDAVService;
use crate::error::{Error, Result};
use std::path::Path;

mod service_identifier;

pub trait ServiceTrait {
    type Configuration: ServiceConfigurationTrait;

    /// Build a new Service instance from the [`Service`]'s [`Configuration`]
    fn new(configuration: Self::Configuration) -> Result<Self>
    where
        Self: Sized;

    fn identifier(&self) -> ServiceIdentifier;
    fn list_files(&self) -> Result<Vec<FileEntry>>;
    fn download(&self, file: FileEntry, destination: &Path) -> Result<()>;
}

pub enum Services {
    DropboxService(DropboxService),
    WebDAVService(WebDAVService),
}

impl ServiceTrait for Services {
    type Configuration = AbstractServiceConfig;

    /// # Panics
    ///
    /// If `configuration` could not be converted into the requested [`Service`]'s Configuration
    fn new(configuration: Self::Configuration) -> Result<Self> {
        match configuration.identifier() {
            ServiceIdentifier::Dropbox => Ok(Services::DropboxService(DropboxService::new(
                <DropboxService as ServiceTrait>::Configuration::from_service_config(
                    configuration,
                )?,
            )?)),
            ServiceIdentifier::WebDAV => Ok(Services::WebDAVService(WebDAVService::new(
                <WebDAVService as ServiceTrait>::Configuration::from_service_config(
                    configuration,
                )?,
            )?)),
        }
    }

    fn identifier(&self) -> ServiceIdentifier {
        match self {
            Services::DropboxService(service) => service.identifier(),
            Services::WebDAVService(service) => service.identifier(),
        }
    }

    fn list_files(&self) -> Result<Vec<FileEntry>, Error> {
        match self {
            Services::DropboxService(service) => service.list_files(),
            Services::WebDAVService(service) => service.list_files(),
        }
    }

    fn download(&self, file: FileEntry, destination: &Path) -> Result<(), Error> {
        match self {
            Services::DropboxService(service) => service.download(file, destination),
            Services::WebDAVService(service) => service.download(file, destination),
        }
    }
}
