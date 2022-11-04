use crate::backend_v2::browser_storage_backend::BrowserStorageBackend;
use crate::browser_storage::BrowserStorage;
use crate::config::Config;
use cqrs::record_trait::RecordTrait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use webchordr_common::session::Session;

#[derive(Default)]
pub struct BrowserStorageBackendFactory {}

impl BrowserStorageBackendFactory {
    pub fn new() -> Self {
        Self {}
    }

    /// ```no_run
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// pub struct Value {
    ///     pub age: i32,
    ///     pub name: String,
    /// }
    /// impl libchordr::prelude::RecordTrait for Value {
    ///     type Id = String;
    ///
    ///     fn id(&self) -> Self::Id {
    ///         self.name.clone()
    ///     }
    /// }
    /// use webchordr_common::config::Config;
    /// use webchordr_common::session::Session;
    /// use webchordr_persistence::backend_v2::browser_storage_backend_factory::BrowserStorageBackendFactory;
    /// use webchordr_persistence::backend_v2::browser_storage_backend::BrowserStorageBackend;
    /// let _storage: BrowserStorageBackend<_,Value> =
    ///             BrowserStorageBackendFactory::default().build(&Config::default(), &Session::default());
    /// ```
    pub fn build<R: RecordTrait + Serialize + DeserializeOwned>(
        &self,
        _config: &Config,
        _session: &Session,
    ) -> BrowserStorageBackend<BrowserStorage, R> {
        let browser_storage = BrowserStorage::local_storage().unwrap();

        BrowserStorageBackend::new(browser_storage)
    }
}
