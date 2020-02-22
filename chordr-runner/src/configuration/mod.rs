//! Configuration file handling
//!
//! This module provides the data structures ([`Configuration`], [`ServiceConfiguration`]) and
//! [`Reader`] to fetch configuration from files
pub(crate) mod reader;

use serde::Deserialize;
use std::path::PathBuf;
use libsynchord::prelude::ServiceIdentifier;

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    /// Path to the catalog file
    pub catalog_file: PathBuf,

    /// Path to the output directory
    pub output_directory: PathBuf,

    /// Online service configuration (dropbox, WebDAV)
    pub service: ServiceConfiguration,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServiceConfiguration {
    /// Online service to use (dropbox, WebDAV)
    pub identifier: ServiceIdentifier,

    /// API key to authenticate with the service (dropbox)
    pub api_token: Option<String>,

    /// Username to authenticate with the service (WebDAV)
    pub username: Option<String>,

    /// Password to authenticate with the service (WebDAV)
    pub password: Option<String>,

    /// WebDAV entry point URL (WebDAV)
    pub url: Option<String>,

    /// Remote directory to list
    pub remote_directory: Option<String>,

    /// Number of seconds to wait between service updates
    pub sync_interval: u64,
}
