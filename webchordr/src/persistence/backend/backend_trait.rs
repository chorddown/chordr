use crate::errors::WebError;
use async_trait::async_trait;
use libchordr::prelude::RecordIdTrait;
use serde::{Deserialize, Serialize};

/// Trait for a persistent data Backend.
///
/// It will take care of storing and loading data from it's Storage (e.g. Browser Storage,
/// Server API)
#[async_trait(? Send)]
pub trait BackendTrait {
    /// Store `value` with the given `key` in the `namespace`
    ///
    /// `value` will be serialized before it is stored
    async fn store<T: Serialize + RecordIdTrait, N: AsRef<str>, K: AsRef<str>>(
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

    // /// Load the stored value with the given `key` in the `namespace`
    // async fn find_by_identifier<T, I, ID, N: AsRef<str>, K: AsRef<str>>(
    //     &self,
    //     namespace: N,
    //     key: K,
    //     identifier: I,
    // ) -> Result<Option<T>, WebError>
    // where
    //     I: Into<<T as RecordIdTrait>::Id>,
    //     T: for<'a> Deserialize<'a> + RecordIdTrait,
    // {
    //     self.load(namespace, key).await
    //     // Err(WebError::custom_error("f"))
    // }
}
