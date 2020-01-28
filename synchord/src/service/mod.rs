mod dropbox_service;
mod file_entry;

pub use self::dropbox_service::DropboxService;
pub use self::file_entry::FileEntry;
use crate::error::{Error, Result};
use std::path::Path;

pub trait ServiceTrait {
    fn list_files(&self) -> Result<Vec<FileEntry>>;
    fn download(&self, file: FileEntry, destination: &Path) -> Result<()>;
}

pub enum Services {
    DropboxService(DropboxService),
}

impl ServiceTrait for Services {
    fn list_files(&self) -> Result<Vec<FileEntry>, Error> {
        match self {
            Services::DropboxService(dropbox_service) => dropbox_service.list_files(),
        }
    }

    fn download(&self, file: FileEntry, destination: &Path) -> Result<(), Error> {
        match self {
            Services::DropboxService(dropbox_service) => {
                dropbox_service.download(file, destination)
            }
        }
    }
}
