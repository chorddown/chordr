use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use async_trait::async_trait;
use log::{error, warn};
use serde::{Deserialize, Serialize};

use cqrs::prelude::{Command, Count, Query};
use libchordr::prelude::RecordTrait;
use webchordr_common::session::Session;
use webchordr_common::tri::Tri;

use crate::backend::BackendTrait;
use crate::backend::CommandQueryBackendTrait;
use crate::errors::WebError;
use crate::lock::Stupex;
use crate::persistence_manager::command_context::CommandContext;
use crate::persistence_manager::server_backend_type::ServerBackendType;

use super::persistence_manager_trait::PersistenceManagerTrait;

type Lock<I> = Stupex<I>;

pub struct PersistenceManager<CB, SB, TB> {
    session: Session,
    client_backend: Arc<Lock<CB>>,
    server_backend: Arc<Lock<SB>>,
    transient_backend: Arc<Lock<TB>>,
}

// async fn backend_call<'r, F, R: 'r, Fu, B: BackendTrait>(
//     hint: &'static str,
//     backend: &'r Arc<Lock<B>>,
//     callback: F,
// ) -> R
// where
//     Fu: Future<Output = R>,
//     F: FnOnce(&B) -> Fu,
// {
//     let locked_backend = backend
//         .lock()
//         .await
//         .expect(&format!("Could not acquire lock for {}", hint));
//
//     callback(&*locked_backend).await
// }

impl<CB: CommandQueryBackendTrait, SB: CommandQueryBackendTrait, TB: CommandQueryBackendTrait>
    PersistenceManager<CB, SB, TB>
{
    pub fn new(
        session: Session,
        client_backend: CB,
        server_backend: SB,
        transient_backend: TB,
    ) -> Self {
        Self {
            session,
            client_backend: Arc::new(Lock::new(client_backend)),
            server_backend: Arc::new(Lock::new(server_backend)),
            transient_backend: Arc::new(Lock::new(transient_backend)),
        }
    }

    async fn perform_command<T: Serialize + RecordTrait>(
        &self,
        command: Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        if let Err(e) = self.client_command(&command).await {
            if self.session.is_unauthenticated() {
                return Err(e);
            } else {
                error!("{}", e)
            }
        }

        if self.session.is_authenticated() {
            self.server_command(&command).await
        } else {
            Ok(())
        }
    }

    async fn server_store<T: Serialize + RecordTrait>(
        &self,
        namespace: &str,
        key: &str,
        value: &T,
    ) -> Result<(), WebError> {
        if self.session.is_unauthenticated() {
            return Ok(());
        }

        match ServerBackendType::from_context(&CommandContext::new(namespace, key)) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (server backend)")
                    .store(namespace, key, value)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .store(namespace, key, value)
                    .await
            }
        }
    }

    async fn server_load<T>(&self, namespace: &str, key: &str) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        match ServerBackendType::from_context(&CommandContext::new(namespace, key)) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (server backend)")
                    .load(namespace, key)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .load(namespace, key)
                    .await
            }
        }
    }

    async fn server_command<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        if self.session.is_unauthenticated() {
            return Ok(());
        }

        match ServerBackendType::from_context(command.context()) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (server backend)")
                    .perform(command)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .perform(command)
                    .await
            }
        }
    }

    async fn server_find_all<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Result<Vec<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        if self.session.is_unauthenticated() {
            return Ok(vec![]);
        }

        match ServerBackendType::from_context(query.context()) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for writing (server backend)")
                    .find_all(query)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for writing (transient server backend)")
                    .find_all(query)
                    .await
            }
        }
    }

    async fn server_find_by_id<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        if self.session.is_unauthenticated() {
            return Tri::None;
        }

        match ServerBackendType::from_context(query.context()) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (server backend)")
                    .find_by_id(query)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .find_by_id(query)
                    .await
            }
        }
    }

    async fn client_store<T: Serialize + RecordTrait>(
        &self,
        namespace: &str,
        key: &str,
        value: &T,
    ) -> Result<(), WebError> {
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for writing (client backend)")
            .store(namespace, key, value)
            .await
    }

    async fn client_command<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for writing (client backend)")
            .perform(command)
            .await
    }

    async fn client_find_all<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Result<Vec<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for writing (client backend)")
            .find_all(query)
            .await
    }

    async fn client_find_by_id<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for writing (client backend)")
            .find_by_id(query)
            .await
    }
}

#[async_trait(? Send)]
impl<CB: CommandQueryBackendTrait, SB: CommandQueryBackendTrait, TB: CommandQueryBackendTrait>
    BackendTrait for PersistenceManager<CB, SB, TB>
{
    async fn store<T: Serialize + RecordTrait, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        // TODO: use join!
        if let Err(e) = self
            .client_store(namespace.as_ref(), key.as_ref(), value)
            .await
        {
            error!("{}", e)
        }

        self.server_store(namespace.as_ref(), key.as_ref(), value)
            .await
    }

    async fn load<T, N: AsRef<str>, K: AsRef<str>>(&self, namespace: N, key: K) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        if self.session.is_authenticated() {
            let server_result = self.server_load(namespace.as_ref(), key.as_ref()).await;
            if let Tri::Err(e) = server_result {
                warn!("{}", e)
            } else {
                return server_result;
            }
        }
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for reading (client backend)")
            .load(namespace.as_ref(), key.as_ref())
            .await
    }
}

#[async_trait(? Send)]
impl<CB: CommandQueryBackendTrait, SB: CommandQueryBackendTrait, TB: CommandQueryBackendTrait>
    PersistenceManagerTrait for PersistenceManager<CB, SB, TB>
{
    async fn find_all<T>(&self, context: CommandContext) -> Result<Vec<T>, WebError>
    where
        T: for<'a> Deserialize<'a> + RecordTrait,
    {
        let query = Query::all(context);

        if self.session.is_authenticated() {
            let server_result = self.server_find_all(&query).await;

            if let Err(e) = server_result {
                warn!("{}", e);
            } else {
                return server_result;
            }
        }

        self.client_find_all(&query).await
    }

    async fn count_all(&self, context: CommandContext) -> Result<Count, WebError> {
        // No real type is needed to perform the counting. All data the backend needs is provided in
        // the Context
        #[derive(Serialize, Deserialize)]
        struct EmptyType {}
        impl RecordTrait for EmptyType {
            type Id = u8;

            fn id(&self) -> Self::Id {
                8
            }
        }

        self.find_all::<EmptyType>(context)
            .await
            .map(|v| v.len() as Count)
    }

    async fn find_by_id<T>(
        &self,
        context: CommandContext,
        id: <T as cqrs::record_trait::RecordTrait>::Id,
    ) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a> + RecordTrait,
    {
        let command = Query::by_id(id, context);
        if self.session.is_authenticated() {
            let server_result = self.server_find_by_id(&command).await;
            if let Tri::Err(e) = server_result {
                // Log the warning and fall through to querying the client backend
                warn!("{}", e);
            } else {
                return server_result;
            }
        }

        self.client_find_by_id(&command).await
    }

    async fn add<'a, T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &'a T,
    ) -> Result<(), WebError>
    where
        &'a T: RecordTrait,
    {
        self.perform_command(Command::add(instance, context)).await
    }

    async fn update<'a, T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &'a T,
    ) -> Result<(), WebError>
    where
        &'a T: RecordTrait,
    {
        self.perform_command(Command::update(instance, context))
            .await
    }

    async fn delete<T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &T,
    ) -> Result<(), WebError> {
        let command: Command<T, _> = Command::delete(instance.id(), context);
        self.perform_command(command).await
    }
}

impl<CB: Debug, SB: Debug, TB: Debug> Debug for PersistenceManager<CB, SB, TB> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn try_lock_or_print_info<T: Debug>(lock: &Arc<Lock<T>>) -> String {
            lock.try_lock()
                .map_or("Can't acquire lock".to_owned(), |i| format!("{:?}", i))
        }
        write!(
            f,
            "PersistenceManager {{
session: {:?},
client_backend: {},
server_backend: {},
transient_backend: {}
}}",
            self.session,
            try_lock_or_print_info(&self.client_backend),
            try_lock_or_print_info(&self.server_backend),
            try_lock_or_print_info(&self.transient_backend),
        )
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use wasm_bindgen_test::*;

    use libchordr::models::setlist::Setlist;

    use crate::backend::{BrowserStorageBackend, TransientBackend};
    use crate::browser_storage::HashMapBrowserStorage;
    use crate::constants::{STORAGE_KEY_SETLIST, TEST_STORAGE_NAMESPACE};
    use crate::prelude::BrowserStorage;
    use crate::shared::hash_map_from_context_and_slice;
    use crate::storage_key_utility::build_combined_id_key;
    use crate::test_helpers::{get_test_command_context, TestValue};
    use crate::test_helpers::{get_test_setlist, get_test_user, get_test_user_password_hidden};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn build_hash_map_persistence_manager() -> PersistenceManager<
        BrowserStorageBackend<HashMapBrowserStorage>,
        TransientBackend,
        TransientBackend,
    > {
        let client_backend = BrowserStorageBackend::new(HashMapBrowserStorage::new());
        let server_backend = TransientBackend::new();
        let transient_backend = TransientBackend::new();

        PersistenceManager::new(
            Session::default(),
            client_backend,
            server_backend,
            transient_backend,
        )
    }

    fn build_transient_backend_with_entries(entries: &[TestValue]) -> TransientBackend {
        TransientBackend::new_with_map(hash_map_from_context_and_slice(
            &CommandContext::new(TEST_STORAGE_NAMESPACE, STORAGE_KEY_SETLIST),
            entries,
        ))
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn store_and_load_i32_test() {
        let pm = build_hash_map_persistence_manager();
        let value: i32 = 12;
        assert!(pm
            .store(TEST_STORAGE_NAMESPACE, "key-1", &value)
            .await
            .is_ok());

        assert!(pm
            .load::<i32, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
            .await
            .is_some());
        assert_eq!(
            value,
            pm.load::<i32, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
                .await
                .unwrap()
        );
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn store_and_load_person_test() {
        let pm = build_hash_map_persistence_manager();
        let value = TestValue {
            age: 3,
            name: "Daniel".to_string(),
        };

        assert!(pm
            .store(TEST_STORAGE_NAMESPACE, "key-1", &value)
            .await
            .is_ok());

        assert!(pm
            .load::<TestValue, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
            .await
            .is_some());
        assert!(pm
            .load::<TestValue, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
            .await
            .is_some());
        assert_eq!(
            value,
            pm.load::<TestValue, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
                .await
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    async fn store_and_load_person_localstorage_test() {
        let client_backend = BrowserStorageBackend::new(
            BrowserStorage::local_storage().expect("Could not create Browser Storage"),
        );
        let pm = PersistenceManager::new(
            Session::default(),
            client_backend,
            TransientBackend::new(),
            TransientBackend::new(),
        );
        let value = TestValue {
            age: 3,
            name: "Daniel".to_string(),
        };

        assert!(pm
            .store(TEST_STORAGE_NAMESPACE, "key-1", &value)
            .await
            .is_ok());

        assert!(pm
            .load::<TestValue, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
            .await
            .is_some());
        assert_eq!(
            value,
            pm.load::<TestValue, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
                .await
                .unwrap()
        );
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn store_and_load_setlist_test() {
        let pm = build_hash_map_persistence_manager();

        let original_value = get_test_setlist(get_test_user());
        let expected_value = get_test_setlist(get_test_user_password_hidden());

        assert!(pm
            .store(TEST_STORAGE_NAMESPACE, "my-setlist", &original_value)
            .await
            .is_ok());

        assert!(pm
            .load::<Setlist, _, _>(TEST_STORAGE_NAMESPACE, "my-setlist")
            .await
            .is_some());
        assert_eq!(
            expected_value,
            pm.load::<Setlist, _, _>(TEST_STORAGE_NAMESPACE, "my-setlist")
                .await
                .unwrap()
        );
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn find_all_in_client_test() {
        let test_values = vec![
            TestValue::new(3, "Daniel"),
            TestValue::new(13, "Peter"),
            TestValue::new(76, "Justin"),
            TestValue::new(6, "Paulina"),
        ];
        let client_backend = build_transient_backend_with_entries(&test_values);
        let server_backend = TransientBackend::new();
        let transient_backend = TransientBackend::new();

        let pm = PersistenceManager::new(
            Session::unauthenticated(),
            client_backend,
            server_backend,
            transient_backend,
        );

        perform_find_all(test_values, pm).await
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn find_all_on_server_test() {
        let server_test_values = vec![
            TestValue::new(3, "Daniel"),
            TestValue::new(13, "Peter"),
            TestValue::new(76, "Justin"),
            TestValue::new(6, "Paulina"),
        ];
        let client_backend = TransientBackend::new();
        let server_backend = build_transient_backend_with_entries(&server_test_values);
        let transient_backend = TransientBackend::new();

        let pm = PersistenceManager::new(
            Session::new_with_user(get_test_user()),
            client_backend,
            server_backend,
            transient_backend,
        );

        perform_find_all(server_test_values, pm).await
    }

    async fn perform_find_all(
        test_values: Vec<TestValue>,
        pm: PersistenceManager<TransientBackend, TransientBackend, TransientBackend>,
    ) {
        let result = pm.find_all::<TestValue>(get_test_command_context()).await;
        assert!(result.is_ok());
        let all = result.unwrap();
        assert_eq!(all.len(), 4);
        for test_value in test_values {
            assert!(all.iter().find(|v| v == &&test_value).is_some())
        }
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn count_all_in_client_test() {
        let test_values = vec![
            TestValue::new(3, "Daniel"),
            TestValue::new(13, "Peter"),
            TestValue::new(76, "Justin"),
            TestValue::new(6, "Paulina"),
        ];
        let client_backend = build_transient_backend_with_entries(&test_values);
        let server_backend = TransientBackend::new();
        let transient_backend = TransientBackend::new();

        let pm = PersistenceManager::new(
            Session::unauthenticated(),
            client_backend,
            server_backend,
            transient_backend,
        );

        let result = pm.count_all(get_test_command_context()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn find_by_id_in_client_test() {
        let test_person = TestValue::new(76, "Justin");
        let client_backend = TransientBackend::new();
        let server_backend = build_transient_backend_with_entries(&[
            TestValue::new(3, "Daniel"),
            TestValue::new(13, "Peter"),
            test_person.clone(),
            TestValue::new(6, "Paulina"),
        ]);
        let transient_backend = TransientBackend::new();

        let pm = PersistenceManager::new(
            Session::new_with_user(get_test_user()),
            client_backend,
            server_backend,
            transient_backend,
        );

        perform_find_by_id(test_person, pm).await;
    }

    //#[wasm_bindgen_test]
    #[tokio::test]
    async fn find_by_id_on_server_test() {
        let test_person = TestValue::new(76, "Justin");
        let client_backend = build_transient_backend_with_entries(&[
            TestValue::new(3, "Daniel"),
            TestValue::new(13, "Peter"),
            test_person.clone(),
            TestValue::new(6, "Paulina"),
        ]);
        let server_backend = TransientBackend::new();
        let transient_backend = TransientBackend::new();

        let pm = PersistenceManager::new(
            Session::unauthenticated(),
            client_backend,
            server_backend,
            transient_backend,
        );

        perform_find_by_id(test_person, pm).await;
    }

    async fn perform_find_by_id(
        test_person: TestValue,
        pm: PersistenceManager<TransientBackend, TransientBackend, TransientBackend>,
    ) {
        let result = pm
            .find_by_id::<TestValue>(get_test_command_context(), "Justin".into())
            .await;
        assert!(result.is_some());
        assert_eq!(result.unwrap(), test_person);
    }

    // #[wasm_bindgen_test]
    #[tokio::test]
    async fn add_test() {
        let test_value = TestValue::new(39, "Thomas");
        let pm = PersistenceManager::new(
            Session::default(),
            TransientBackend::new(),
            TransientBackend::new(),
            TransientBackend::new(),
        );
        let result = pm
            .add::<TestValue>(get_test_command_context(), &test_value)
            .await;
        assert!(result.is_ok(), "{}", result.unwrap_err());

        let PersistenceManager { client_backend, .. } = pm;
        let client_backend = client_backend
            .lock()
            .await
            .expect("Could not acquire lock (client backend)");
        assert_eq!(client_backend.data().len(), 1);
        assert!(client_backend
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
        let client_backend = build_transient_backend_with_entries(&test_values);
        assert_eq!(client_backend.data().len(), 4);
        let pm = PersistenceManager::new(
            Session::default(),
            client_backend,
            TransientBackend::new(),
            TransientBackend::new(),
        );
        assert!(pm
            .update::<TestValue>(get_test_command_context(), &updated_value)
            .await
            .is_ok());
        let PersistenceManager { client_backend, .. } = pm;
        let client_backend = client_backend
            .lock()
            .await
            .expect("Could not acquire lock (client backend)");
        let data = client_backend.data();
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
        let client_backend = build_transient_backend_with_entries(&[value_to_delete.clone()]);
        assert_eq!(client_backend.data().len(), 1);
        let pm = PersistenceManager::new(
            Session::default(),
            client_backend,
            TransientBackend::new(),
            TransientBackend::new(),
        );
        assert!(pm
            .delete::<TestValue>(get_test_command_context(), &value_to_delete)
            .await
            .is_ok());
        let PersistenceManager { client_backend, .. } = pm;
        let client_backend = client_backend
            .lock()
            .await
            .expect("Could not acquire lock (client backend)");
        let data = client_backend.data();
        assert!(data.is_empty());
        let option = data.get(&build_combined_id_key::<TestValue>(
            &get_test_command_context(),
            &value_to_delete.id(),
        ));
        assert!(option.is_none());
    }
}
