use crate::backend::BackendTrait;
use crate::browser_storage::*;
use crate::errors::PersistenceError;
use crate::errors::WebError;
use crate::storage_key_utility::build_combined_key;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub struct BrowserStorageBackend<B> {
    browser_storage: Arc<RwLock<B>>,
}

impl<B: BrowserStorageTrait> BrowserStorageBackend<B> {
    pub fn new(browser_storage: B) -> Self {
        Self {
            browser_storage: Arc::new(RwLock::new(browser_storage)),
        }
    }
}

#[async_trait(? Send)]
impl<B: BrowserStorageTrait> BackendTrait for BrowserStorageBackend<B> {
    async fn store<T: Serialize, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        match serde_json::to_string(&value) {
            Ok(serialized) => self
                .browser_storage
                .write()
                .expect("Could not acquire lock for writing")
                .set_item(build_combined_key(&namespace, &key), serialized),
            Err(e) => Err(PersistenceError::serialization_error(e.to_string()).into()),
        }
    }

    async fn load<T, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
    ) -> Result<Option<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        match self
            .browser_storage
            .read()
            .expect("Could not acquire lock for reading")
            .get_item(build_combined_key(&namespace, &key))
        {
            Some(v) => match serde_json::from_str(v.as_str()) {
                Ok(serialized) => Ok(serialized),
                Err(e) => Err(PersistenceError::deserialization_error(e, Some(v)).into()),
            },
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::{
        get_test_setlist, get_test_user, get_test_user_password_hidden, TestValue,
    };
    use libchordr::models::setlist::Setlist;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn store_and_load_i32_test() {
        let pm = BrowserStorageBackend::new(HashMapBrowserStorage::new());
        let value: i32 = 12;
        assert!(pm.store("test", "key-1", &value).await.is_ok());

        assert!(pm.load::<i32, _, _>("test", "key-1").await.is_ok());
        assert!(pm
            .load::<i32, _, _>("test", "key-1")
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            value,
            pm.load::<i32, _, _>("test", "key-1")
                .await
                .unwrap()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    async fn store_and_load_person_test() {
        let pm = BrowserStorageBackend::new(HashMapBrowserStorage::new());
        let value = TestValue {
            age: 3,
            name: "Daniel".to_string(),
        };

        assert!(pm.store("test", "key-1", &value).await.is_ok());

        assert!(pm.load::<TestValue, _, _>("test", "key-1").await.is_ok());
        assert!(pm
            .load::<TestValue, _, _>("test", "key-1")
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            value,
            pm.load::<TestValue, _, _>("test", "key-1")
                .await
                .unwrap()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    async fn store_and_load_person_localstorage_test() {
        let pm = BrowserStorageBackend::new(
            BrowserStorage::local_storage().expect("Could not create Browser Storage"),
        );
        let value = TestValue {
            age: 3,
            name: "Daniel".to_string(),
        };

        assert!(pm.store("test", "key-1", &value).await.is_ok());

        assert!(pm.load::<TestValue, _, _>("test", "key-1").await.is_ok());
        assert!(pm
            .load::<TestValue, _, _>("test", "key-1")
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            value,
            pm.load::<TestValue, _, _>("test", "key-1")
                .await
                .unwrap()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    async fn store_and_load_setlist_test() {
        let pm = BrowserStorageBackend::new(HashMapBrowserStorage::new());

        let original_value = get_test_setlist(get_test_user());
        let expected_value = get_test_setlist(get_test_user_password_hidden());

        assert!(pm
            .store("test", "my-setlist", &original_value)
            .await
            .is_ok());

        assert!(pm.load::<Setlist, _, _>("test", "my-setlist").await.is_ok());
        assert!(pm
            .load::<Setlist, _, _>("test", "my-setlist")
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            expected_value,
            pm.load::<Setlist, _, _>("test", "my-setlist")
                .await
                .unwrap()
                .unwrap()
        );
    }
}
