use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use cqrs::prelude::{Command, Query, RecordTrait};
use webchordr_common::tri::Tri;

use crate::backend::{
    BackendTrait, CommandBackendTrait, CommandQueryBackendTrait, QueryBackendTrait,
};
use crate::browser_storage::*;
use crate::errors::PersistenceError;
use crate::errors::WebError;
use crate::persistence_manager::CommandContext;
use crate::shared::{
    deserialize_value, missing_record_id_error, record_not_found_error, store_with_command,
    ExistenceCheck,
};
use crate::storage_key_utility::{build_combined_id_key, build_combined_key, SEPARATOR};

pub struct BrowserStorageBackend<B> {
    browser_storage: Arc<RwLock<B>>,
}

impl<B: BrowserStorageTrait> BrowserStorageBackend<B> {
    pub fn new(browser_storage: B) -> Self {
        Self {
            browser_storage: Arc::new(RwLock::new(browser_storage)),
        }
    }

    /// Add or Update the data according to the given command
    ///
    /// The two commands are basically only different whether the record should already exist in the
    /// database
    fn store_with_command<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
        existence_check: ExistenceCheck,
    ) -> Result<(), WebError> {
        store_with_command(
            command,
            existence_check,
            |combined_id_key| match self.lock_for_reading() {
                Ok(l) => l.get_item(combined_id_key).is_some(),
                Err(_) => false,
            },
            |combined_id_key, serialized_value| {
                self.browser_storage
                    .write()
                    .expect("Could not acquire lock for writing")
                    .set_item(combined_id_key, serialized_value)
            },
        )
    }

    /// Acquire a lock for reading
    fn lock_for_reading(&self) -> Result<RwLockReadGuard<B>, WebError> {
        match self.browser_storage.read() {
            Ok(l) => Ok(l),
            Err(_) => {
                Err(PersistenceError::general_error("Could not acquire lock for reading").into())
            }
        }
    }

    /// Acquire a lock for reading
    fn lock_for_writing(&self) -> Result<RwLockWriteGuard<B>, WebError> {
        match self.browser_storage.write() {
            Ok(l) => Ok(l),
            Err(_) => {
                Err(PersistenceError::general_error("Could not acquire lock for writing").into())
            }
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
                .lock_for_writing()?
                .set_item(build_combined_key(&namespace, &key), serialized),
            Err(e) => Err(PersistenceError::serialization_error(e.to_string()).into()),
        }
    }

    async fn load<T, N: AsRef<str>, K: AsRef<str>>(&self, namespace: N, key: K) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let lock_guard = match self.lock_for_reading() {
            Ok(l) => l,
            Err(e) => return Tri::Err(e),
        };

        match lock_guard.get_item(build_combined_key(&namespace, &key)) {
            Some(v) => match serde_json::from_str(v.as_str()) {
                Ok(serialized) => Tri::from_option(serialized),
                Err(e) => Tri::Err(PersistenceError::deserialization_error(e, Some(v)).into()),
            },
            None => Tri::None,
        }
    }
}

#[async_trait(? Send)]
impl<B: BrowserStorageTrait> CommandBackendTrait for BrowserStorageBackend<B> {
    async fn add<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        self.store_with_command(command, ExistenceCheck::MustNotExist)
    }

    async fn update<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        self.store_with_command(command, ExistenceCheck::MustExist)
    }

    async fn delete<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        let id = match command.id() {
            None => return Err(missing_record_id_error()),
            Some(r) => r,
        };
        let combined_id_key = build_combined_id_key::<T>(command.context(), id);
        let entry_does_exist = self
            .lock_for_reading()?
            .get_item(&combined_id_key)
            .is_some();
        if !entry_does_exist {
            return Err(record_not_found_error::<T>(id));
        }

        self.lock_for_writing()?.remove_item(combined_id_key)
    }
}

#[async_trait(? Send)]
impl<B: BrowserStorageTrait> QueryBackendTrait for BrowserStorageBackend<B> {
    async fn find_all<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Result<Vec<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let combined_key = format!(
            "{}{}",
            build_combined_key(&query.context().namespace, &query.context().key),
            SEPARATOR
        );

        let storage = self.lock_for_reading()?;
        Ok(storage
            .keys()
            .iter()
            .filter_map(|key| {
                // Check if the current key starts with the combined key
                if !key.starts_with(&combined_key) {
                    return None;
                }
                let serialized = storage.get_item(key)?;
                if let Tri::Some(deserialized) = deserialize_value(&serialized) {
                    deserialized
                } else {
                    None
                }
            })
            .collect())
    }

    async fn find_by_id<T: RecordTrait>(&self, query: &Query<T, CommandContext>) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let id = match query.id() {
            None => return Tri::Err(missing_record_id_error()),
            Some(r) => r,
        };
        let combined_id_key = build_combined_id_key::<T>(query.context(), id);

        let lock_guard = match self.lock_for_reading() {
            Ok(l) => l,
            Err(e) => return Tri::Err(e),
        };
        match lock_guard.get_item(&combined_id_key) {
            Some(v) => deserialize_value(&v),
            None => Tri::None,
        }
    }
}

impl<B: BrowserStorageTrait> CommandQueryBackendTrait for BrowserStorageBackend<B> {}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use wasm_bindgen_test::*;

    use cqrs::prelude::Query;
    use libchordr::models::setlist::Setlist;

    use crate::shared::hash_map_from_context_and_slice;
    use crate::storage_key_utility::build_combined_id_key;
    use crate::test_helpers::{
        get_test_command_context, get_test_setlist, get_test_user, get_test_user_password_hidden,
        TestValue,
    };

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn store_and_load_i32_test() {
        let pm = BrowserStorageBackend::new(HashMapBrowserStorage::new());
        let value: i32 = 12;
        assert!(pm.store("test", "key-1", &value).await.is_ok());

        assert!(pm.load::<i32, _, _>("test", "key-1").await.is_some());
        assert_eq!(value, pm.load::<i32, _, _>("test", "key-1").await.unwrap());
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn store_and_load_person_test() {
        let pm = BrowserStorageBackend::new(HashMapBrowserStorage::new());
        let value = TestValue {
            age: 3,
            name: "Daniel".to_string(),
        };

        assert!(pm.store("test", "key-1", &value).await.is_ok());

        assert!(pm.load::<TestValue, _, _>("test", "key-1").await.is_some());
        assert_eq!(
            value,
            pm.load::<TestValue, _, _>("test", "key-1").await.unwrap()
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

        assert!(pm.load::<TestValue, _, _>("test", "key-1").await.is_some());
        assert_eq!(
            value,
            pm.load::<TestValue, _, _>("test", "key-1").await.unwrap()
        );
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn store_and_load_setlist_test() {
        let pm = BrowserStorageBackend::new(HashMapBrowserStorage::new());

        let original_value = get_test_setlist(get_test_user());
        let expected_value = get_test_setlist(get_test_user_password_hidden());

        assert!(pm
            .store("test", "my-setlist", &original_value)
            .await
            .is_ok());

        assert!(pm
            .load::<Setlist, _, _>("test", "my-setlist")
            .await
            .is_some());
        assert_eq!(
            expected_value,
            pm.load::<Setlist, _, _>("test", "my-setlist")
                .await
                .unwrap()
        );
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn find_all_test() {
        let test_values = vec![
            TestValue::new(3, "Daniel"),
            TestValue::new(13, "Peter"),
            TestValue::new(76, "Justin"),
            TestValue::new(6, "Paulina"),
        ];
        let backend = BrowserStorageBackend::new(HashMapBrowserStorage::new_with_hash_map(
            hash_map_from_context_and_slice(&get_test_command_context(), &test_values),
        ));

        let result = backend
            .find_all::<TestValue>(&Query::all(get_test_command_context()))
            .await;
        assert!(result.is_ok());
        let all = result.unwrap();
        assert_eq!(all.len(), 4);
        for test_value in test_values {
            assert!(all.iter().find(|v| v == &&test_value).is_some())
        }
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn find_by_id_test() {
        let test_person = TestValue::new(76, "Justin");
        let backend = BrowserStorageBackend::new(HashMapBrowserStorage::new_with_hash_map(
            hash_map_from_context_and_slice(
                &get_test_command_context(),
                &[
                    TestValue::new(3, "Daniel"),
                    TestValue::new(13, "Peter"),
                    test_person.clone(),
                    TestValue::new(6, "Paulina"),
                ],
            ),
        ));
        let result = backend
            .find_by_id::<TestValue>(&Query::by_id("Justin".into(), get_test_command_context()))
            .await;
        assert!(result.is_some());
        assert_eq!(result.unwrap(), test_person);
    }

    // #[wasm_bindgen_test]
    #[tokio::test]
    async fn add_test() {
        let test_value = TestValue::new(39, "Thomas");
        let backend = BrowserStorageBackend::new(HashMapBrowserStorage::new());
        let result = backend
            .add::<TestValue>(&Command::add(
                test_value.clone(),
                get_test_command_context(),
            ))
            .await;
        assert!(result.is_ok(), "{}", result.unwrap_err());

        let BrowserStorageBackend { browser_storage } = backend;
        let browser_storage = browser_storage
            .read()
            .expect("Could not acquire lock for reading");
        assert_eq!(browser_storage.data().len(), 1);
        assert!(browser_storage
            .data()
            .get(&build_combined_id_key::<TestValue>(
                &get_test_command_context(),
                &test_value.id(),
            ))
            .is_some());
    }

    // #[wasm_bindgen_test]
    #[tokio::test]
    async fn update_test() {
        let updated_value = TestValue::new(39, "Thomas");
        let test_values = vec![
            TestValue::new(3, "Daniel"),
            TestValue::new(38, "Thomas"),
            TestValue::new(76, "Justin"),
            TestValue::new(6, "Paulina"),
        ];
        let storage = HashMapBrowserStorage::new_with_hash_map(hash_map_from_context_and_slice(
            &get_test_command_context(),
            &test_values,
        ));
        assert_eq!(storage.data().len(), 4);

        let backend = BrowserStorageBackend::new(storage);
        assert!(backend
            .update(&Command::update(
                updated_value.clone(),
                get_test_command_context()
            ))
            .await
            .is_ok());
        let BrowserStorageBackend { browser_storage } = backend;
        let browser_storage = browser_storage
            .read()
            .expect("Could not acquire lock for reading");
        let data = browser_storage.data();
        assert_eq!(data.len(), 4);
        let option = data.get(&build_combined_id_key::<TestValue>(
            &get_test_command_context(),
            &updated_value.id(),
        ));
        assert!(option.is_some());
        assert_eq!(
            option.unwrap(),
            &serde_json::to_string(&updated_value).unwrap()
        );
    }

    // #[wasm_bindgen_test]
    #[tokio::test]
    async fn delete() {
        let value_to_delete = TestValue::new(3, "Daniel");
        let storage = HashMapBrowserStorage::new_with_hash_map(hash_map_from_context_and_slice(
            &get_test_command_context(),
            &[value_to_delete.clone()],
        ));

        assert_eq!(storage.data().len(), 1);
        let backend = BrowserStorageBackend::new(storage);

        assert!(backend
            .delete::<TestValue>(&Command::delete(
                value_to_delete.id(),
                get_test_command_context()
            ))
            .await
            .is_ok());
        let BrowserStorageBackend { browser_storage } = backend;
        let browser_storage = browser_storage
            .read()
            .expect("Could not acquire lock for reading");
        let data = browser_storage.data();
        assert!(data.is_empty());
        let option = data.get(&build_combined_id_key::<TestValue>(
            &get_test_command_context(),
            &value_to_delete.id(),
        ));
        assert!(option.is_none());
    }
}
