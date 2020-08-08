use crate::errors::WebError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// The Persistence Manager will take care of storing and loading data.
///
/// The manager may use different backends for storing and loading data
/// and is responsible for synchronization of those.
#[async_trait(? Send)]
pub trait PersistenceManagerTrait {
    /// Store `value` with the given `key` in the `namespace`
    ///
    /// `value` will be serialized before it is stored
    async fn store<T: Serialize, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError>;

    /// Load the stored value with the given `key` in the `namespace`
    async fn load<T, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
    ) -> Result<Option<T>, WebError>
        where
            T: for<'a> Deserialize<'a>;
}
