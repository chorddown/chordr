use std::cell::RefCell;
use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use cqrs::prelude::{Command, Query};
use libchordr::prelude::RecordTrait;
use webchordr_common::tri::Tri;

#[allow(deprecated)]
use crate::backend::backend_trait::{CommandBackendTrait, QueryBackendTrait};
#[allow(deprecated)]
use crate::backend::{BackendTrait, CommandQueryBackendTrait};
use crate::command_context::CommandContext;
use crate::errors::WebError;
use crate::shared::{
    deserialize_value, missing_record_id_error, record_not_found_error, store_with_command,
    ExistenceCheck,
};
use crate::storage_key_utility::{build_combined_id_key, build_combined_key, SEPARATOR};

type Data = HashMap<String, String>;

#[derive(Debug)]
pub struct TransientBackend {
    data: RefCell<Data>,
}

impl TransientBackend {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_with_map(map: Data) -> Self {
        Self {
            data: RefCell::new(map),
        }
    }

    /// Allow access to the data
    #[cfg(test)]
    pub(crate) fn data(&self) -> std::cell::Ref<Data> {
        self.data.borrow()
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
            |combined_id_key| self.data.borrow().get(combined_id_key).is_some(),
            |combined_id_key, serialized_value| {
                self.data
                    .borrow_mut()
                    .insert(combined_id_key, serialized_value);
                Ok(())
            },
        )
    }
}

impl Default for TransientBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(deprecated)]
#[async_trait(? Send)]
impl BackendTrait for TransientBackend {
    async fn store<T: Serialize + RecordTrait, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        self.data.borrow_mut().insert(
            build_combined_key(&namespace.as_ref(), &key.as_ref()),
            serde_json::to_string(value).expect("Could not serialize"),
        );
        Ok(())
    }

    async fn load<T, N: AsRef<str>, K: AsRef<str>>(&self, namespace: N, key: K) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        match self
            .data
            .borrow()
            .get(&build_combined_key(&namespace.as_ref(), &key.as_ref()))
        {
            Some(v) => deserialize_value(v.as_str()),
            None => Tri::None,
        }
    }
}

#[allow(deprecated)]
#[async_trait(? Send)]
impl CommandBackendTrait for TransientBackend {
    async fn upsert<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        self.store_with_command(command, ExistenceCheck::DoNotCheck)
    }

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
        let id = &command.record().id();
        let combined_id_key = build_combined_id_key::<T>(command.context(), id);

        let mut borrowed = self.data.borrow_mut();
        if borrowed.get(&combined_id_key).is_none() {
            return Err(record_not_found_error::<T>(id));
        }
        borrowed.remove(&combined_id_key);
        Ok(())
    }
}

#[allow(deprecated)]
#[async_trait(? Send)]
impl QueryBackendTrait for TransientBackend {
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

        Ok(self
            .data
            .borrow()
            .iter()
            .filter_map(|(key, serialized)| {
                // Check if the current key starts with the combined key
                if !key.starts_with(&combined_key) {
                    return None;
                }
                if let Tri::Some(deserialized) = deserialize_value(serialized) {
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

        match self.data.borrow().get(&combined_id_key) {
            Some(v) => deserialize_value(v),
            None => Tri::None,
        }
    }
}

impl CommandQueryBackendTrait for TransientBackend {}
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
        let pm = TransientBackend::new();
        let value: i32 = 12;
        assert!(pm.store("test", "key-1", &value).await.is_ok());

        assert!(pm.load::<i32, _, _>("test", "key-1").await.is_some());
        assert_eq!(value, pm.load::<i32, _, _>("test", "key-1").await.unwrap());
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn store_and_load_person_test() {
        let pm = TransientBackend::new();
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
        let pm = TransientBackend::new();

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
        let backend = TransientBackend::new_with_map(hash_map_from_context_and_slice(
            &get_test_command_context(),
            &test_values,
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
        let backend = TransientBackend::new_with_map(hash_map_from_context_and_slice(
            &get_test_command_context(),
            &[
                TestValue::new(3, "Daniel"),
                TestValue::new(13, "Peter"),
                test_person.clone(),
                TestValue::new(6, "Paulina"),
            ],
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
        let backend = TransientBackend::new();
        let result = backend
            .add::<TestValue>(&Command::add(
                test_value.clone(),
                get_test_command_context(),
            ))
            .await;
        assert!(result.is_ok(), "{}", result.unwrap_err());

        assert_eq!(backend.data().len(), 1);
        assert!(backend
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
        let backend = TransientBackend::new_with_map(hash_map_from_context_and_slice(
            &get_test_command_context(),
            &test_values,
        ));
        assert_eq!(backend.data().len(), 4);

        assert!(backend
            .update(&Command::update(
                updated_value.clone(),
                get_test_command_context()
            ))
            .await
            .is_ok());

        let data = backend.data();
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
        let backend = TransientBackend::new_with_map(hash_map_from_context_and_slice(
            &get_test_command_context(),
            &[value_to_delete.clone()],
        ));

        assert_eq!(backend.data().len(), 1);

        let result = backend
            .delete::<TestValue>(&Command::delete(
                value_to_delete.clone(),
                get_test_command_context(),
            ))
            .await;
        assert!(result.is_ok(), "{}", result.unwrap_err());

        let data = backend.data();
        assert!(data.is_empty());
        let option = data.get(&build_combined_id_key::<TestValue>(
            &get_test_command_context(),
            &value_to_delete.id(),
        ));
        assert!(option.is_none());
    }
}
