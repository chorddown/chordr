use web_sys::Storage;

use crate::errors::PersistenceError;
use crate::errors::WebError;
use crate::helpers::window;

use super::BrowserStorageTrait;

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
    fn keys(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(self.len());
        for n in 0..=self.len() {
            match self.storage.key(n as u32) {
                Ok(i) if i.is_some() => result.push(i.unwrap()),
                Ok(_) => { /* should not happen */ }
                Err(_) => {}
            };
        }
        result
    }

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
#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use wasm_bindgen_test::*;

    use webchordr_common::constants::TEST_STORAGE_NAMESPACE;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn clear_test_data(storage: &mut BrowserStorage) {
        let prefix = get_test_prefix();
        storage.remove_item(format!("{}.A", prefix)).unwrap();
        storage.remove_item(format!("{}.B", prefix)).unwrap();
        storage.remove_item(format!("{}.C", prefix)).unwrap();
    }

    fn get_test_prefix() -> String {
        format!("{}.test-browser-storage", TEST_STORAGE_NAMESPACE,)
    }

    #[wasm_bindgen_test]
    fn set_and_get_item_test() {
        let prefix = get_test_prefix();

        let mut storage = BrowserStorage::session_storage().expect("Could not get session storage");
        clear_test_data(&mut storage);

        assert!(storage.set_item(format!("{}.A", prefix), "Apple").is_ok());
        assert!(storage.set_item(format!("{}.B", prefix), "Banana").is_ok());
        assert_eq!(
            storage.get_item(format!("{}.A", prefix)),
            Some("Apple".to_owned())
        );
        assert_eq!(
            storage.get_item(format!("{}.B", prefix)),
            Some("Banana".to_owned())
        );
        assert_eq!(storage.get_item(format!("{}.C", prefix)), None);

        assert!(storage
            .set_item(format!("{}.B", prefix), "Blueberries")
            .is_ok());
        assert_eq!(
            storage.get_item(format!("{}.B", prefix)),
            Some("Blueberries".to_owned())
        );
        clear_test_data(&mut storage);
    }

    #[wasm_bindgen_test]
    fn remove_item_test() {
        let mut storage = BrowserStorage::session_storage().expect("Could not get session storage");
        clear_test_data(&mut storage);

        let prefix = get_test_prefix();
        let key = format!("{}.A", prefix);
        assert!(storage.set_item(key.clone(), "Apple").is_ok());
        assert_eq!(storage.get_item(&key), Some("Apple".to_owned()));
        assert!(storage.remove_item(&key).is_ok());
        assert_eq!(storage.get_item(&key), None);
        clear_test_data(&mut storage);
    }

    #[wasm_bindgen_test]
    fn len_test() {
        let prefix = get_test_prefix();
        let key_a = format!("{}.A", prefix);
        let key_b = format!("{}.B", prefix);

        let mut storage = BrowserStorage::session_storage().expect("Could not get session storage");
        clear_test_data(&mut storage);
        let initial_len = storage.len();
        assert_eq!(storage.len(), initial_len + 0);
        assert!(storage.set_item(key_a.clone(), "Apple").is_ok());
        assert_eq!(storage.len(), initial_len + 1);
        assert!(storage.set_item(key_b.clone(), "Banana").is_ok());
        assert_eq!(storage.len(), initial_len + 2);
        assert!(storage.set_item(key_b, "Something new for B").is_ok());
        assert_eq!(storage.len(), initial_len + 2);
        assert!(storage.remove_item(key_a).is_ok());
        assert_eq!(storage.len(), initial_len + 1);
        clear_test_data(&mut storage);
    }

    #[wasm_bindgen_test]
    fn keys_test() {
        let mut storage = BrowserStorage::session_storage().expect("Could not get session storage");
        clear_test_data(&mut storage);
        let prefix = get_test_prefix();
        assert!(storage.set_item(format!("{}.A", prefix), "Apple").is_ok());
        assert!(storage.set_item(format!("{}.B", prefix), "Banana").is_ok());
        assert!(storage.set_item(format!("{}.C", prefix), "Citron").is_ok());

        let keys = storage.keys();
        for key in ["A", "B", "C"] {
            assert!(keys.contains(&format!("{}.{}", prefix, key)))
        }
        clear_test_data(&mut storage);
    }
}
