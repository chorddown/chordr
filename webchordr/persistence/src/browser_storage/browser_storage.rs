use super::BrowserStorageTrait;
use crate::errors::PersistenceError;
use crate::errors::WebError;
use crate::helpers::window;
use web_sys::Storage;

pub enum BrowserStorageType {
    LocalStorage,
    SessionStorage,
}

#[derive(Clone)]
pub struct BrowserStorage {
    storage: Storage,
}

impl BrowserStorage {
    pub fn local_storage() -> Result<Self, WebError> {
        Self::new(BrowserStorageType::LocalStorage)
    }

    pub fn session_storage() -> Result<Self, WebError> {
        Self::new(BrowserStorageType::SessionStorage)
    }

    pub fn new(browser_storage_type: BrowserStorageType) -> Result<Self, WebError> {
        let storage_option = match browser_storage_type {
            BrowserStorageType::LocalStorage => window().local_storage()?,
            BrowserStorageType::SessionStorage => window().session_storage()?,
        };
        match storage_option {
            Some(storage) => Ok(Self { storage }),
            None => Err(PersistenceError::storage_unavailable(
                "Could not retrieve browser storage",
            )
            .into()),
        }
    }
}

impl BrowserStorageTrait for BrowserStorage {
    fn get_item<S: AsRef<str>>(&self, key_name: S) -> Option<String> {
        match self.storage.get_item(key_name.as_ref()) {
            Ok(v) => v,
            Err(_) => None,
        }
    }

    fn set_item<S: Into<String>, V: Into<String>>(
        &mut self,
        key_name: S,
        key_value: V,
    ) -> Result<(), WebError> {
        Ok(self
            .storage
            .set_item(key_name.into().as_ref(), key_value.into().as_ref())?)
    }

    fn remove_item<S: AsRef<str>>(&mut self, key_name: S) -> Result<(), WebError> {
        Ok(self.storage.remove_item(key_name.as_ref())?)
    }

    fn clear(&mut self) -> Result<(), WebError> {
        Ok(self.storage.clear()?)
    }

    fn len(&self) -> usize {
        match self.storage.length() {
            Ok(l) => l as usize,
            Err(_) => 0,
        }
    }
}
