use crate::errors::PersistenceError;
use crate::errors::WebError;
use crate::persistence::backend::BackendTrait;
use crate::persistence::browser_storage::*;
use async_trait::async_trait;
use log::debug;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub struct DummyServerBackend {
    data: Arc<RwLock<HashMapBrowserStorage>>,
}

impl DummyServerBackend {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMapBrowserStorage::new())),
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
            .data
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
