use crate::backend::BrowserStorageBackend;
use crate::browser_storage::BrowserStorage;
use crate::config::Config;
use webchordr_common::session::Session;

pub struct BrowserStorageBackendFactory {}

impl BrowserStorageBackendFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(
        &self,
        _config: &Config,
        _session: &Session,
    ) -> BrowserStorageBackend<BrowserStorage> {
        let browser_storage = BrowserStorage::local_storage().unwrap();

        BrowserStorageBackend::new(browser_storage)
    }
}
