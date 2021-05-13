use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use chrono::DateTime;
use dropbox_sdk::client_trait::{Endpoint, HttpClient, HttpRequestResultRaw, ParamsType, Style};
use dropbox_sdk::files::{DownloadArg, FileMetadata, ListFolderArg, Metadata};
use dropbox_sdk::UserAuthClient;

use crate::error::{Error, Result};
use crate::service::file_entry::FileEntry;
use crate::service::{
    AbstractServiceConfig, ServiceConfigurationTrait, ServiceIdentifier, ServiceTrait,
};

pub struct DropboxService {
    http_client: Box<dyn dropbox_sdk::client_trait::UserAuthClient>,
    // http_client: Box<dyn dropbox_sdk::client_trait::HttpClient>,
}

impl DropboxService {
    fn fetch_file_stream(&self, file: String) -> Result<Box<dyn Read>> {
        let request_argument = DownloadArg::new(file);

        let result = dropbox_sdk::files::download(&self, &request_argument, None, None)??;

        match result.body {
            Some(body) => Ok(body),
            None => Err(Error::download_error("Response body is empty")),
        }
    }
}

pub struct DropboxServiceConfiguration {
    api_key: String,
    local_directory: PathBuf,
}

impl ServiceConfigurationTrait for DropboxServiceConfiguration {
    fn from_service_config(service_config: AbstractServiceConfig) -> Result<Self> {
        Ok(Self {
            api_key: service_config.api_key()?,
            local_directory: service_config.local_directory().to_path_buf(),
        })
    }

    fn identifier(&self) -> ServiceIdentifier {
        ServiceIdentifier::Dropbox
    }

    fn local_directory(&self) -> &Path {
        self.local_directory.as_path()
    }
}

impl ServiceTrait for DropboxService {
    type Configuration = DropboxServiceConfiguration;

    fn new(configuration: Self::Configuration) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            http_client: Box::new(dropbox_sdk::default_client::UserAuthDefaultClient::new(
                configuration.api_key,
            )),
        })
    }

    fn identifier(&self) -> ServiceIdentifier {
        ServiceIdentifier::Dropbox
    }

    /// List files inside "Apps/synchord"
    ///
    /// Once a new App has been created inside the Dropbox developer UI the token can be retrieved
    /// and the "App folder name" for the newly created app can be defined. With the app token the
    /// service will only have access to the contents of the App folder.
    ///
    /// https://www.dropbox.com/developers/apps
    fn list_files(&self) -> Result<Vec<FileEntry>, Error> {
        let path_relative_to_app_folder = "".to_owned();
        let request_argument: ListFolderArg = ListFolderArg::new(path_relative_to_app_folder);
        let result = dropbox_sdk::files::list_folder(&self, &request_argument)??;

        Ok(result
            .entries
            .iter()
            .filter_map(|m| {
                match m {
                    Metadata::File(data) => FileEntry::try_from(data).ok(),
                    Metadata::Folder(_) => return None, // Not implemented
                    Metadata::Deleted(_) => return None, // Not implemented
                }
            })
            .collect())
    }

    fn download(&self, file: FileEntry, destination: &Path) -> Result<()> {
        let mut body = self.fetch_file_stream(file.path().to_owned())?;
        let mut file_handle = File::create(destination)?;
        io::copy(&mut body, &mut file_handle)?;

        Ok(())
    }
}

impl HttpClient for &DropboxService {
    fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params: String,
        params_type: ParamsType,
        body: Option<&[u8]>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> dropbox_sdk::Result<HttpRequestResultRaw> {
        self.http_client.request(
            endpoint,
            style,
            function,
            params,
            params_type,
            body,
            range_start,
            range_end,
        )
    }
}

impl UserAuthClient for &DropboxService {}

impl TryFrom<&FileMetadata> for FileEntry {
    type Error = ();

    fn try_from(value: &FileMetadata) -> Result<Self, Self::Error> {
        let path = match value.path_lower {
            Some(ref p) => p,
            None => return Err(()),
        };

        match DateTime::parse_from_rfc3339(&value.server_modified) {
            Ok(date) => Ok(FileEntry::new(path, value.size as usize, date)),
            Err(_) => Err(()),
        }
    }
}
