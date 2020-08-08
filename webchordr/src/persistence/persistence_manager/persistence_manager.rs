use super::persistence_manager_trait::PersistenceManagerTrait;
use crate::errors::WebError;
use crate::persistence::backend::BackendTrait;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub struct PersistenceManager<B> {
    backend: Arc<RwLock<B>>,
}

impl<B: BackendTrait> PersistenceManager<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend: Arc::new(RwLock::new(backend)),
        }
    }
}

#[async_trait(? Send)]
impl<B: BackendTrait> BackendTrait for PersistenceManager<B> {
    async fn store<T: Serialize, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        self.backend
            .write()
            .expect("Could not acquire lock for writing")
            .store(namespace, key, value)
            .await
    }

    async fn load<T, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
    ) -> Result<Option<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        self.backend
            .read()
            .expect("Could not acquire lock for reading")
            .load(namespace, key)
            .await
    }
}

#[async_trait(? Send)]
impl<B: BackendTrait> PersistenceManagerTrait for PersistenceManager<B> {}

#[cfg(test)]
mod test {
    use super::*;
    use serde::{Deserialize, Serialize};
    use wasm_bindgen_test::*;

    use crate::persistence::backend::BrowserStorageBackend;
    use crate::persistence::browser_storage::HashMapBrowserStorage;
    use crate::persistence::prelude::BrowserStorage;
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
        let pm = PersistenceManager::new(BrowserStorageBackend::new(HashMapBrowserStorage::new()));
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
        let pm = PersistenceManager::new(BrowserStorageBackend::new(HashMapBrowserStorage::new()));
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
        let backend = BrowserStorageBackend::new(
            BrowserStorage::new().expect("Could not create Browser Storage"),
        );
        let pm = PersistenceManager::new(backend);
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
        let pm = PersistenceManager::new(BrowserStorageBackend::new(HashMapBrowserStorage::new()));

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
