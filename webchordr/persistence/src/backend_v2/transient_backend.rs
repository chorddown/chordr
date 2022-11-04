use crate::errors::WebError;
use crate::persistence_manager::CommandContext;
use crate::shared::*;
use crate::storage_key_utility::{build_combined_id_key, build_combined_key, SEPARATOR};
use async_trait::async_trait;
use cqrs::nonblocking::{CommandExecutor, QueryExecutor};
use cqrs::prelude::{Command, Query};
use libchordr::prelude::RecordTrait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use webchordr_common::tri::Tri;

type Data = HashMap<String, String>;

#[derive(Debug, Default)]
pub struct TransientBackend<R: RecordTrait + Serialize + DeserializeOwned> {
    data: RefCell<Data>,
    _data_type: PhantomData<R>,
}

impl<R: RecordTrait + Serialize + DeserializeOwned> TransientBackend<R> {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
            _data_type: PhantomData,
        }
    }

    pub fn new_with_map(map: Data) -> Self {
        Self {
            data: RefCell::new(map),
            _data_type: PhantomData,
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
    fn store_with_command(
        &self,
        command: &Command<R, CommandContext>,
        existence_check: ExistenceCheck,
    ) -> Result<(), WebError> {
        store_with_command(
            &command,
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

#[async_trait(? Send)]
impl<R: RecordTrait + Serialize + DeserializeOwned> CommandExecutor for TransientBackend<R> {
    type RecordType = R;
    type Error = WebError;
    type Context = CommandContext;

    async fn upsert(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        self.store_with_command(command, ExistenceCheck::DoNotCheck)
    }

    async fn add(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        self.store_with_command(command, ExistenceCheck::MustNotExist)
    }

    async fn update(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        self.store_with_command(command, ExistenceCheck::MustExist)
    }

    async fn delete(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        let id = &command.record().id();
        let combined_id_key = build_combined_id_key::<R>(command.context(), id);

        let mut borrowed = self.data.borrow_mut();
        if borrowed.get(&combined_id_key).is_none() {
            return Err(record_not_found_error::<Self::RecordType>(id));
        }
        borrowed.remove(&combined_id_key);
        Ok(())
    }
}

#[async_trait(? Send)]
impl<R: RecordTrait + Serialize + DeserializeOwned> QueryExecutor for TransientBackend<R> {
    type RecordType = R;
    type Error = WebError;
    type Context = CommandContext;

    async fn find_all(
        &self,
        query: &Query<Self::RecordType, Self::Context>,
    ) -> Result<Vec<Self::RecordType>, Self::Error> {
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

    async fn find_by_id(
        &self,
        query: &Query<Self::RecordType, Self::Context>,
    ) -> Tri<Self::RecordType, Self::Error> {
        let id = match query.id() {
            None => return Tri::Err(missing_record_id_error()),
            Some(r) => r,
        };
        let combined_id_key = build_combined_id_key::<Self::RecordType>(query.context(), id);

        match self.data.borrow().get(&combined_id_key) {
            Some(v) => deserialize_value(v),
            None => Tri::None,
        }
    }
}

// impl<R: RecordTrait + Serialize + DeserializeOwned> CommandQueryBackendTrait for TransientBackend<R> {}

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
        let backend: TransientBackend<TestValue> = TransientBackend::new_with_map(
            hash_map_from_context_and_slice(&get_test_command_context(), &test_values),
        );

        let result = backend
            .find_all(&Query::all(get_test_command_context()))
            .await;
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
        let backend: TransientBackend<TestValue> =
            TransientBackend::new_with_map(hash_map_from_context_and_slice(
                &get_test_command_context(),
                &[
                    TestValue::new(3, "Daniel"),
                    TestValue::new(13, "Peter"),
                    test_person.clone(),
                    TestValue::new(6, "Paulina"),
                ],
            ));
        let result = backend
            .find_by_id(&Query::by_id("Justin".into(), get_test_command_context()))
            .await;
        assert!(result.is_some());
        assert_eq!(result.unwrap(), test_person);
    }

    #[tokio::test]
    async fn add_test() {
        let test_value = TestValue::new(39, "Thomas");
        let backend: TransientBackend<TestValue> = TransientBackend::new();
        let result = backend
            .add(&Command::add(
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

    #[tokio::test]
    async fn update_test() {
        let updated_value = TestValue::new(39, "Thomas");
        let test_values = vec![
            TestValue::new(3, "Daniel"),
            TestValue::new(38, "Thomas"),
            TestValue::new(76, "Justin"),
            TestValue::new(6, "Paulina"),
        ];
        let backend: TransientBackend<TestValue> = TransientBackend::new_with_map(
            hash_map_from_context_and_slice(&get_test_command_context(), &test_values),
        );
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

    #[tokio::test]
    async fn delete() {
        let value_to_delete = TestValue::new(3, "Daniel");
        let backend: TransientBackend<TestValue> =
            TransientBackend::new_with_map(hash_map_from_context_and_slice(
                &get_test_command_context(),
                &[value_to_delete.clone()],
            ));

        assert_eq!(backend.data().len(), 1);

        let result = backend
            .delete(&Command::delete(
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
