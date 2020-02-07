mod dropbox_service;
mod file_entry;
mod web_dav_service;

pub use self::dropbox_service::DropboxService;
pub use self::file_entry::FileEntry;
pub use self::web_dav_service::WebDAVService;
use crate::error::{Error, Result};
use std::path::Path;

pub trait ServiceTrait {
    fn list_files(&self) -> Result<Vec<FileEntry>>;
    fn download(&self, file: FileEntry, destination: &Path) -> Result<()>;
}

pub enum Services {
    DropboxService(DropboxService),
    WebDAVService(WebDAVService),
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
