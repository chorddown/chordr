use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::errors::PersistenceError;
use crate::errors::WebError;
use crate::persistence::backend::BackendTrait;
use crate::persistence::browser_storage::*;
use crate::persistence::storage_key_utility::build_combined_key;

pub struct DummyServerBackend {
    data: Arc<RwLock<HashMapBrowserStorage>>,
}

impl DummyServerBackend {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMapBrowserStorage::new())),
        }
    }
}

#[async_trait(? Send)]
impl BackendTrait for DummyServerBackend {
    async fn store<T: Serialize, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        debug!("x store");
        match serde_json::to_string(&value) {
            Ok(serialized) => self
                .data
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
            .data
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
