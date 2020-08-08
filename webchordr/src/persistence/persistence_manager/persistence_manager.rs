use crate::errors::PersistenceError;
use crate::errors::WebError;
use crate::persistence::browser_storage::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use super::persistence_manager_trait::PersistenceManagerTrait;

pub struct PersistenceManager<B> {
    browser_storage: Arc<RwLock<B>>,
}

impl<B: BrowserStorageTrait> PersistenceManager<B> {
    pub fn new(browser_storage: B) -> Self {
        Self {
            browser_storage: Arc::new(RwLock::new(browser_storage)),
        }
    }

    fn build_combined_key<N: AsRef<str>, K: AsRef<str>>(&self, namespace: &N, key: &K) -> String {
        if namespace.as_ref().is_empty() {
            key.as_ref().to_string()
        } else {
            format!("{}.{}", namespace.as_ref(), key.as_ref())
        }
    }
}

#[async_trait(? Send)]
impl<B: BrowserStorageTrait> PersistenceManagerTrait for PersistenceManager<B> {
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
                .set_item(self.build_combined_key(&namespace, &key), serialized),
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
            .get_item(self.build_combined_key(&namespace, &key))
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
    use serde::{Deserialize, Serialize};
    use wasm_bindgen_test::*;

    use crate::test_helpers::{entry, get_test_user};
    use chrono::prelude::*;
    use libchordr::models::setlist::Setlist;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestValue {
        pub age: i32,
        pub name: String,
    }

    #[wasm_bindgen_test]
    async fn store_and_load_i32_test() {
        let pm = PersistenceManager::new(HashMapBrowserStorage::new());
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
        let pm = PersistenceManager::new(HashMapBrowserStorage::new());
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
        let pm = PersistenceManager::new(
            BrowserStorage::new().expect("Could not create Browser Storage"),
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
        let pm = PersistenceManager::new(HashMapBrowserStorage::new());

        let value = Setlist::new(
            "My setlist",
            10291,
            get_test_user(),
            None,
            Some(Utc.ymd(2014, 11, 14).and_hms(8, 9, 10)),
            Utc.ymd(2020, 06, 14).and_hms(16, 26, 20),
            Utc::now(),
            vec![entry("song-1"), entry("song-2"), entry("song-3")],
        );

        assert!(pm.store("test", "my-setlist", &value).await.is_ok());

        assert!(pm.load::<Setlist, _, _>("test", "my-setlist").await.is_ok());
        assert!(pm
            .load::<Setlist, _, _>("test", "my-setlist")
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            value,
            pm.load::<Setlist, _, _>("test", "my-setlist")
                .await
                .unwrap()
                .unwrap()
        );
    }
}
