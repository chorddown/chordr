use crate::config::Config;
use crate::persistence::backend::BrowserStorageBackend;
use crate::persistence::prelude::BrowserStorage;
use crate::session::Session;

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
