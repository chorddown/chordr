use super::browser_storage_trait::BrowserStorageTrait;
use crate::errors::WebError;
use std::collections::HashMap;

pub struct HashMapBrowserStorage {
    map: HashMap<String, String>,
}

impl HashMapBrowserStorage {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl BrowserStorageTrait for HashMapBrowserStorage {
    fn get_item<S: AsRef<str>>(&self, key_name: S) -> Option<String> {
        self.map.get(key_name.as_ref()).cloned()
    }

    fn set_item<S: Into<String>, V: Into<String>>(
        &mut self,
        key_name: S,
        key_value: V,
    ) -> Result<(), WebError> {
        self.map.insert(key_name.into(), key_value.into());
        Ok(())
    }

    fn remove_item<S: AsRef<str>>(&mut self, key_name: S) -> Result<(), WebError> {
        self.map.remove(key_name.as_ref());
        Ok(())
    }

    fn clear(&mut self) -> Result<(), WebError> {
        self.map.clear();
        Ok(())
    }

    fn len(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_and_get_item_test() {
        let mut storage = HashMapBrowserStorage::new();
        assert!(storage.set_item("A", "Apple").is_ok());
        assert!(storage.set_item("B", "Banana").is_ok());
        assert_eq!(storage.get_item("A"), Some("Apple".to_owned()));
        assert_eq!(storage.get_item("B"), Some("Banana".to_owned()));
        assert_eq!(storage.get_item("C"), None);

        assert!(storage.set_item("B", "Blueberries").is_ok());
        assert_eq!(storage.get_item("B"), Some("Blueberries".to_owned()));
    }

    #[test]
    fn remove_item_test() {
        let mut storage = HashMapBrowserStorage::new();
        assert!(storage.set_item("A", "Apple").is_ok());
        assert_eq!(storage.get_item("A"), Some("Apple".to_owned()));
        assert!(storage.remove_item("A").is_ok());
        assert_eq!(storage.get_item("A"), None);
    }

    #[test]
    fn clear_test() {
        let mut storage = HashMapBrowserStorage::new();
        assert!(storage.set_item("A", "Apple").is_ok());
        assert_eq!(storage.get_item("A"), Some("Apple".to_owned()));
        assert!(storage.clear().is_ok());
        assert_eq!(storage.get_item("A"), None);
        assert_eq!(storage.len(), 0);
    }

    #[test]
    fn len_test() {
        let mut storage = HashMapBrowserStorage::new();
        assert_eq!(storage.len(), 0);
        assert!(storage.set_item("A", "Apple").is_ok());
        assert_eq!(storage.len(), 1);
        assert!(storage.set_item("B", "Banana").is_ok());
        assert_eq!(storage.len(), 2);
        assert!(storage.set_item("B", "Something new for B").is_ok());
        assert_eq!(storage.len(), 2);
        assert!(storage.remove_item("A").is_ok());
        assert_eq!(storage.len(), 1);
    }
}
