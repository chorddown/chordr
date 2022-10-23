use crate::browser_storage::*;
use crate::errors::PersistenceError;
use crate::errors::WebError;
use crate::persistence_manager::CommandContext;
use crate::shared::{
    deserialize_value, missing_record_id_error, record_not_found_error, store_with_command,
    ExistenceCheck,
};
use crate::storage_key_utility::{build_combined_id_key, build_combined_key, SEPARATOR};
use cqrs::blocking::CommandExecutor;
use cqrs::blocking::QueryExecutor;
use cqrs::prelude::{Command, Query, RecordTrait};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use webchordr_common::tri::Tri;

pub struct BrowserStorageBackend<B, R: RecordTrait + Serialize + DeserializeOwned> {
    browser_storage: Rc<RwLock<B>>,
    _data_type: PhantomData<R>,
}

impl<B: BrowserStorageTrait, R: RecordTrait + Serialize + DeserializeOwned>
    BrowserStorageBackend<B, R>
{
    pub fn new(browser_storage: B) -> Self {
        Self {
            browser_storage: Rc::new(RwLock::new(browser_storage)),
            _data_type: PhantomData,
        }
    }

    /// Add or Update the data according to the given command
    ///
    /// The two commands are basically only different whether the record should already exist in the
    /// database
    fn store_with_command(
        &self,
        command: Command<<Self as CommandExecutor>::RecordType, CommandContext>,
        existence_check: ExistenceCheck,
    ) -> Result<(), WebError> {
        store_with_command(
            &command,
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

    /// Acquire a lock for writing
    fn lock_for_writing(&self) -> Result<RwLockWriteGuard<B>, WebError> {
        match self.browser_storage.write() {
            Ok(l) => Ok(l),
            Err(_) => {
                Err(PersistenceError::general_error("Could not acquire lock for writing").into())
            }
        }
    }
}

impl<B: BrowserStorageTrait, R: RecordTrait + Serialize + DeserializeOwned> CommandExecutor
    for BrowserStorageBackend<B, R>
{
    type RecordType = R;
    type Error = WebError;
    type Context = CommandContext;

    fn upsert(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), WebError> {
        self.store_with_command(command, ExistenceCheck::DoNotCheck)
    }

    fn add(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), WebError> {
        self.store_with_command(command, ExistenceCheck::MustNotExist)
    }

    fn update(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), WebError> {
        self.store_with_command(command, ExistenceCheck::MustExist)
    }

    fn delete(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), WebError> {
        let id = &command.record().id();
        let combined_id_key = build_combined_id_key::<Self::RecordType>(command.context(), id);
        let entry_does_exist = self
            .lock_for_reading()?
            .get_item(&combined_id_key)
            .is_some();
        if !entry_does_exist {
            return Err(record_not_found_error::<Self::RecordType>(id));
        }

        self.lock_for_writing()?.remove_item(combined_id_key)
    }
}
impl<B: BrowserStorageTrait, R: RecordTrait + Serialize + DeserializeOwned> QueryExecutor
    for BrowserStorageBackend<B, R>
{
    type RecordType = R;
    type Error = WebError;
    type Context = CommandContext;

    fn find_all(
        &self,
        query: Query<Self::RecordType, Self::Context>,
    ) -> Result<Vec<Self::RecordType>, Self::Error> {
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

    fn find_by_id(
        &self,
        query: Query<Self::RecordType, Self::Context>,
    ) -> Tri<Self::RecordType, Self::Error> {
        let id = match query.id() {
            None => return Tri::Err(missing_record_id_error()),
            Some(r) => r,
        };
        let combined_id_key = build_combined_id_key::<Self::RecordType>(query.context(), id);

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::shared::hash_map_from_context_and_slice;
    use crate::storage_key_utility::build_combined_id_key;
    use crate::test_helpers::{get_test_command_context, TestValue};
    use cqrs::prelude::Query;

    #[tokio::test]
    async fn find_all_test() {
        let test_values = vec![
            TestValue::new(3, "Daniel"),
            TestValue::new(13, "Peter"),
            TestValue::new(76, "Justin"),
            TestValue::new(6, "Paulina"),
        ];
        let backend: BrowserStorageBackend<_, TestValue> =
            BrowserStorageBackend::new(HashMapBrowserStorage::new_with_hash_map(
                hash_map_from_context_and_slice(&get_test_command_context(), &test_values),
            ));

        let result = backend.find_all(Query::all(get_test_command_context()));
        assert!(result.is_ok());
        let all = result.unwrap();
        assert_eq!(all.len(), 4);
        for test_value in test_values {
            assert!(all.iter().find(|v| v == &&test_value).is_some())
        }
    }

    #[tokio::test]
    async fn find_by_id_test() {
        let test_person = TestValue::new(76, "Justin");
        let backend: BrowserStorageBackend<_, TestValue> = BrowserStorageBackend::new(
            HashMapBrowserStorage::new_with_hash_map(hash_map_from_context_and_slice(
                &get_test_command_context(),
                &[
                    TestValue::new(3, "Daniel"),
                    TestValue::new(13, "Peter"),
                    test_person.clone(),
                    TestValue::new(6, "Paulina"),
                ],
            )),
        );
        let result = backend.find_by_id(Query::by_id("Justin".into(), get_test_command_context()));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), test_person);
    }

    #[tokio::test]
    async fn add_test() {
        let test_value = TestValue::new(39, "Thomas");
        let backend: BrowserStorageBackend<_, TestValue> =
            BrowserStorageBackend::new(HashMapBrowserStorage::new());
        let result = backend.add(Command::add(test_value.clone(), get_test_command_context()));
        assert!(result.is_ok(), "{}", result.unwrap_err());

        let BrowserStorageBackend {
            browser_storage, ..
        } = backend;
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
            .update(Command::update(
                updated_value.clone(),
                get_test_command_context()
            ))
            .is_ok());
        let BrowserStorageBackend {
            browser_storage, ..
        } = backend;
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

    #[tokio::test]
    async fn delete() {
        let value_to_delete = TestValue::new(3, "Daniel");
        let storage = HashMapBrowserStorage::new_with_hash_map(hash_map_from_context_and_slice(
            &get_test_command_context(),
            &[value_to_delete.clone()],
        ));

        assert_eq!(storage.data().len(), 1);
        let backend = BrowserStorageBackend::new(storage);

        let result = backend.delete(Command::delete(
            value_to_delete.clone(),
            get_test_command_context(),
        ));
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let BrowserStorageBackend {
            browser_storage, ..
        } = backend;
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
