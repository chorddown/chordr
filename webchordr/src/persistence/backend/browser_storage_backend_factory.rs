use crate::persistence::backend::BrowserStorageBackend;
use crate::persistence::prelude::BrowserStorage;

pub struct BrowserStorageBackendFactory {}

impl BrowserStorageBackendFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> BrowserStorageBackend<BrowserStorage> {
        let browser_storage = BrowserStorage::new().unwrap();

        BrowserStorageBackend::new(browser_storage)
    }
}
