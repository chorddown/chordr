use super::persistence_manager_trait::PersistenceManagerTrait;
use crate::constants::{
    STORAGE_KEY_SETLIST, STORAGE_KEY_SETTINGS, STORAGE_NAMESPACE, TEST_STORAGE_NAMESPACE,
};
use crate::errors::WebError;
use crate::lock::Stupex;
use crate::persistence::backend::BackendTrait;
use crate::session::Session;
use async_trait::async_trait;
use libchordr::prelude::RecordTrait;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

type Lock<I> = Stupex<I>;

pub struct PersistenceManager<CB, SB, TB> {
    session: Session,
    client_backend: Arc<Lock<CB>>,
    server_backend: Arc<Lock<SB>>,
    transient_backend: Arc<Lock<TB>>,
}

impl<CB: BackendTrait, SB: BackendTrait, TB: BackendTrait> PersistenceManager<CB, SB, TB> {
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

    async fn store_on_client<T: Serialize + RecordTrait>(
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

    async fn store_on_server<T: Serialize + RecordTrait>(
        &self,
        namespace: &str,
        key: &str,
        value: &T,
    ) -> Result<(), WebError> {
        if namespace != STORAGE_NAMESPACE && namespace != TEST_STORAGE_NAMESPACE {
            panic!("No server backend found for namespace: '{}'", namespace)
        }
        if self.session.is_unauthenticated() {
            return Ok(());
        }

        match key {
            STORAGE_KEY_SETLIST => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for writing (server backend)")
                    .store(namespace, key, value)
                    .await
            }
            STORAGE_KEY_SETTINGS => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for writing (transient server backend)")
                    .store(namespace, key, value)
                    .await
            }
            _ => {
                warn!("No server backend found for key: '{}'", key);
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for writing (transient server backend)")
                    .store(namespace, key, value)
                    .await
            }
        }
    }

    async fn load_from_server<T>(&self, namespace: &str, key: &str) -> Result<Option<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        if namespace != STORAGE_NAMESPACE && namespace != TEST_STORAGE_NAMESPACE {
            panic!("No server backend found for namespace: '{}'", namespace)
        }

        match key {
            STORAGE_KEY_SETLIST => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (server backend)")
                    .load(namespace, key)
                    .await
            }
            STORAGE_KEY_SETTINGS => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .load(namespace, key)
                    .await
            }
            _ => {
                warn!("No server backend found for key: '{}'", key);
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .load(namespace, key)
                    .await
            }
        }
    }
}

#[async_trait(? Send)]
impl<CB: BackendTrait, SB: BackendTrait, TB: BackendTrait> BackendTrait
    for PersistenceManager<CB, SB, TB>
{
    async fn store<T: Serialize + RecordTrait, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        // TODO: use join!
        if let Err(e) = self
            .store_on_client(namespace.as_ref(), key.as_ref(), value)
            .await
        {
            error!("{}", e)
        }

        self.store_on_server(namespace.as_ref(), key.as_ref(), value)
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
        if self.session.is_authenticated() {
            let server_result = self
                .load_from_server(namespace.as_ref(), key.as_ref())
                .await;
            match server_result {
                Ok(v) => return Ok(v),
                Err(e) => warn!("{}", e),
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
impl<CB: BackendTrait, SB: BackendTrait, TB: BackendTrait> PersistenceManagerTrait
    for PersistenceManager<CB, SB, TB>
{
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::persistence::backend::{BrowserStorageBackend, TransientBackend};
    use crate::persistence::browser_storage::HashMapBrowserStorage;
    use crate::persistence::prelude::BrowserStorage;
    use crate::test_helpers::{get_test_setlist, get_test_user, get_test_user_password_hidden};
    use crate::test_helpers::{DummyServerBackend, TestValue};
    use libchordr::models::setlist::Setlist;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    fn build_hash_map_persistence_manager() -> PersistenceManager<
        BrowserStorageBackend<HashMapBrowserStorage>,
        DummyServerBackend,
        TransientBackend,
    > {
        let client_backend = BrowserStorageBackend::new(HashMapBrowserStorage::new());
        let server_backend = DummyServerBackend::new();
        let transient_backend = TransientBackend::new();

        PersistenceManager::new(
            Session::default(),
            client_backend,
            server_backend,
            transient_backend,
        )
    }

    #[wasm_bindgen_test]
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
            .is_ok());
        assert!(pm
            .load::<i32, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            value,
            pm.load::<i32, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
                .await
                .unwrap()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
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
            .is_ok());
        assert!(pm
            .load::<TestValue, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            value,
            pm.load::<TestValue, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
                .await
                .unwrap()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    async fn store_and_load_person_localstorage_test() {
        let backend = BrowserStorageBackend::new(
            BrowserStorage::local_storage().expect("Could not create Browser Storage"),
        );
        let pm = PersistenceManager::new(
            Session::default(),
            backend,
            DummyServerBackend::new(),
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
            .is_ok());
        assert!(pm
            .load::<TestValue, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            value,
            pm.load::<TestValue, _, _>(TEST_STORAGE_NAMESPACE, "key-1")
                .await
                .unwrap()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
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
            .is_ok());
        assert!(pm
            .load::<Setlist, _, _>(TEST_STORAGE_NAMESPACE, "my-setlist")
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            expected_value,
            pm.load::<Setlist, _, _>(TEST_STORAGE_NAMESPACE, "my-setlist")
                .await
                .unwrap()
                .unwrap()
        );
    }
}
