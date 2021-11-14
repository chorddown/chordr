use crate::backend::BackendTrait;
use crate::errors::{PersistenceError, WebError};
use async_trait::async_trait;
use libchordr::prelude::RecordTrait;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct TransientBackend {
    map: RefCell<HashMap<String, String>>,
}

impl TransientBackend {
    pub fn new() -> Self {
        Self {
            map: RefCell::new(HashMap::new()),
        }
    }
}

impl Default for TransientBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(? Send)]
impl BackendTrait for TransientBackend {
    async fn store<T: Serialize + RecordTrait, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        self.map.borrow_mut().insert(
            format!("{}/{}", namespace.as_ref(), key.as_ref()),
            serde_json::to_string(value).expect("Could not serialize"),
        );
        Ok(())
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
            .map
            .borrow()
            .get(&format!("{}/{}", namespace.as_ref(), key.as_ref()))
        {
            Some(v) => match serde_json::from_str(v.as_str()) {
                Ok(serialized) => Ok(serialized),
                Err(e) => Err(PersistenceError::deserialization_error(e, Some(v.clone())).into()),
            },
            None => Ok(None),
        }
    }
}
