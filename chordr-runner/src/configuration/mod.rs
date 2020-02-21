pub(crate) mod reader;

use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::Deserialize;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    /// Path to the catalog file
    pub catalog_file: PathBuf,

    /// Path to the output directory
    pub output_directory: PathBuf,

    /// Online service configuration (dropbox, WebDAV)
    pub service: ServiceConfiguration,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum ServiceIdentifier {
    WebDAV,
    Dropbox,
}

impl Display for ServiceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                ServiceIdentifier::WebDAV => "WebDAV",
                ServiceIdentifier::Dropbox => "Dropbox",
            }
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServiceConfiguration {
    /// Online service to use (dropbox, WebDAV)
    pub identifier: ServiceIdentifier,

    /// API key to authenticate with the service (dropbox)
    pub api_token: String,

    /// Username to authenticate with the service (WebDAV)
    pub username: String,

    /// Password to authenticate with the service (WebDAV)
    pub password: String,

    /// WebDAV entry point URL (WebDAV)
    pub url: String,

    /// Remote directory to list
    pub remote_directory: String,

    /// Number of seconds to wait between service updates
    pub sync_interval: u64,
}
