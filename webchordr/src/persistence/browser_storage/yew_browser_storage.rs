use super::BrowserStorageTrait;
use crate::WebError;
use yew::services::storage::Area;
use yew::services::StorageService;

pub struct YewBrowserStorage {
    storage_service: StorageService,
}

impl YewBrowserStorage {
    pub fn new(area: Area) -> Self {
        Self {
            storage_service: StorageService::new(area).expect("Could not build Storage Service"),
        }
    }
}

impl BrowserStorageTrait for YewBrowserStorage {
    fn get_item<S: AsRef<str>>(&self, key_name: S) -> Option<String> {
        self.storage_service
            .restore::<Result<String, _>>(key_name.as_ref())
            .ok()
    }

    fn set_item<S: Into<String>, V: Into<String>>(
        &mut self,
        key_name: S,
        key_value: V,
    ) -> Result<(), WebError> {
        self.storage_service
            .store(key_name.into().as_str(), Ok(key_value.into()));
        Ok(())
    }

    fn remove_item<S: AsRef<str>>(&mut self, key_name: S) -> Result<(), WebError> {
        self.storage_service.remove(key_name.as_ref());
        Ok(())
    }

    fn clear(&mut self) -> Result<(), WebError> {
        unimplemented!("The Yew Browser Storage does not support clear")
    }

    fn len(&self) -> usize {
        unimplemented!("The Yew Browser Storage does not support len")
    }
}
