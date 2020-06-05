pub mod browser_storage;

use crate::errors::WebError;
use crate::persistence::browser_storage::*;
use serde::{Deserialize, Serialize};

/// The Persistence Manager will take care of storing and loading data.
///
/// The manager may use different backends for storing and loading data
/// and is responsible for synchronization of those.
pub trait PersistenceManagerTrait {
    // /// Store `value` with the given `key` in the `namespace`
    // ///
    // /// `value` will be serialized before it is stored
    // async fn store<T: Serialize, N: AsRef<str>, K: AsRef<str>>(
    //     &mut self,
    //     namespace: N,
    //     key: K,
    //     value: &T,
    // ) -> Result<(), WebError>;
    //
    // /// Load the stored value with the given `key` in the `namespace`
    // async fn load<T, N: AsRef<str>, K: AsRef<str>>(
    //     &mut self,
    //     namespace: N,
    //     key: K,
    // ) -> Result<Option<T>, WebError>
    // where
    //     T: for<'a> Deserialize<'a>;
}

#[derive(Clone)]
pub struct PersistenceManager<B> {
    browser_storage: B,
}

impl<B: BrowserStorageTrait> PersistenceManager<B> {
    pub fn new(browser_storage: B) -> Self {
        Self { browser_storage }
    }

    pub async fn store<T: Serialize, N: AsRef<str>, K: AsRef<str>>(
        &mut self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        match serde_json::to_string(&value) {
            Ok(serialized) => self
                .browser_storage
                .set_item(self.build_combined_key(&namespace, &key), serialized),
            Err(e) => Err(WebError::persistence_error(e.to_string())),
        }
    }

    pub async fn load<T, N: AsRef<str>, K: AsRef<str>>(
        &mut self,
        namespace: N,
        key: K,
    ) -> Result<Option<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        match self
            .browser_storage
            .get_item(self.build_combined_key(&namespace, &key))
        {
            Some(v) => match serde_json::from_str(v.as_str()) {
                Ok(serialized) => Ok(serialized),
                Err(e) => Err(WebError::persistence_error(e.to_string())),
            },
            None => Ok(None),
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

impl<B: BrowserStorageTrait> PersistenceManagerTrait for PersistenceManager<B> {
    // async fn store<T: Serialize, N: AsRef<str>, K: AsRef<str>>(
    //     &mut self,
    //     namespace: N,
    //     key: K,
    //     value: &T,
    // ) -> Result<(), WebError> {
    //     PersistenceManager::store(&self, namespace, key, value)
    // }
    //
    // async fn load<T, N: AsRef<str>, K: AsRef<str>>(
    //     &mut self,
    //     namespace: N,
    //     key: K,
    // ) -> Result<Option<T>, WebError>
    // where
    //     T: for<'a> Deserialize<'a>,
    // {
    //     PersistenceManager::load(&self, namespace, key)
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::{Deserialize, Serialize};
    use wasm_bindgen_test::*;

    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestValue {
        pub age: i32,
        pub name: String,
    }

    #[wasm_bindgen_test]
    async fn store_and_load_i32_test() {
        let mut pm = PersistenceManager::new(HashMapBrowserStorage::new());
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
        let mut pm = PersistenceManager::new(HashMapBrowserStorage::new());
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
        let mut pm = PersistenceManager::new(
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
}
