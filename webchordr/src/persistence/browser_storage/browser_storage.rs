use super::BrowserStorageTrait;
use crate::helpers::window;
use crate::WebError;
use web_sys::Storage;

#[derive(Clone)]
pub struct BrowserStorage {
    storage: Storage,
}

impl BrowserStorage {
    pub fn new() -> Result<Self, WebError> {
        let storage_option = window().local_storage()?;
        match storage_option {
            Some(storage) => Ok(Self { storage }),
            None => Err(WebError::persistence_error(
                "Could not build retrieve browser storage",
            )),
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
