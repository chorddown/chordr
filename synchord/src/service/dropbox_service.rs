use crate::service::ServiceTrait;
use crate::error::{Result, Error};
use std::path::Path;
use std::io::prelude::*;
use dropbox_sdk::files::{DownloadArg, ListFolderArg, Metadata};
use std::fs::File;

pub struct DropboxService {
    http_client: Box<dyn dropbox_sdk::client_trait::HttpClient>
}

impl DropboxService {
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            http_client: Box::new(dropbox_sdk::HyperClient::new(api_key.into()))
        }
    }
}

impl ServiceTrait for DropboxService {
    /// List files inside "Apps/synchord"
    fn list_files(&self) -> Result<Vec<String>, Error> {
        let request_argument: ListFolderArg = ListFolderArg::new("".to_owned());
        let result = dropbox_sdk::files::list_folder(self.http_client.as_ref(), &request_argument)??;

        Ok(
            result.entries
                .iter()
                .filter_map(|m| {
                    match m {
                        Metadata::File(data) => data.path_lower.clone(),
                        Metadata::Folder(_) => return None,
                        Metadata::Deleted(_) => return None,
                    }
                })
                .collect()
        )
    }

    fn download(&self, file: String, destination: &Path) -> Result<(), Error> {
        let body = self.fetch_file(file)?;

        let mut file = File::create(destination)?;
        Ok(file.write_all(body.as_bytes())?)
    }
}

impl DropboxService {
    fn fetch_file(&self, file: String) -> Result<String> {
        let request_argument = DownloadArg::new(file);
        let result = dropbox_sdk::files::download(self.http_client.as_ref(), &request_argument, None, None)??;
        let mut body = String::new();
        result.body.unwrap().read_to_string(&mut body)?;

        Ok(body)
    }
}
