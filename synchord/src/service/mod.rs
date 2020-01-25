mod dropbox_service;

pub use self::dropbox_service::DropboxService;
use crate::error::{Result, Error};
use std::path::Path;

pub type RemotePath = String;

pub trait ServiceTrait {
    fn list_files(&self) -> Result<Vec<RemotePath>>;
    fn download(&self, file: RemotePath, destination: &Path) -> Result<()>;
}

pub enum Services {
    DropboxService(DropboxService),
}

impl ServiceTrait for Services {
    fn list_files(&self) -> Result<Vec<String>, Error> {
        match self {
            Services::DropboxService(dropbox_service) => dropbox_service.list_files(),
        }
    }

    fn download(&self, file: String, destination: &Path) -> Result<(), Error> {
        match self {
            Services::DropboxService(dropbox_service) => dropbox_service.download(file, destination),
        }
    }
}
