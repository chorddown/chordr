mod dropbox_service;
mod file_entry;
mod service_config;
mod web_dav_service;

pub use self::dropbox_service::DropboxService;
pub use self::file_entry::FileEntry;
pub use self::service_config::ServiceConfig;
pub use self::web_dav_service::WebDAVService;
use crate::error::{Error, Result};
pub use crate::service::service_config::ServiceConfigTrait;
use std::path::Path;

pub trait ServiceTrait {
    fn list_files(&self) -> Result<Vec<FileEntry>>;
    fn download(&self, file: FileEntry, destination: &Path) -> Result<()>;
}

pub enum Services {
    DropboxService(DropboxService),
    WebDAVService(WebDAVService),
}

impl Services {
    pub fn build_service_by_identifier<S: ServiceConfigTrait>(
        service_identifier: &str,
        service_config: &S,
    ) -> Result<Self> {
        match service_identifier.to_lowercase().as_str() {
            "dropbox" => Ok(Services::DropboxService(DropboxService::new(
                service_config.api_key()?,
            ))),
            "webdav" => Ok(Services::WebDAVService(WebDAVService::new(
                service_config.url()?,
                service_config.remote_directory()?,
                service_config.username()?,
                service_config.password()?,
            )?)),
            _ => Err(Error::unknown_service_error(format!(
                "Service {} is not implemented",
                service_identifier
            ))),
        }
    }
}

impl ServiceTrait for Services {
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
