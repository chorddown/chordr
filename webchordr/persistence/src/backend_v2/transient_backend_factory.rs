use crate::backend_v2::transient_backend::TransientBackend;
use crate::config::Config;
use cqrs::prelude::RecordTrait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use webchordr_common::session::Session;

#[derive(Default)]
pub struct TransientBackendFactory {}

impl TransientBackendFactory {
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
    /// use webchordr_persistence::backend_v2::transient_backend_factory::TransientBackendFactory;
    /// use webchordr_persistence::backend_v2::transient_backend::TransientBackend;
    /// let _storage: TransientBackend<Value> =
    ///             TransientBackendFactory::default().build(&Config::default(), &Session::default());
    /// ```
    pub fn build<R: RecordTrait + Serialize + DeserializeOwned>(
        &self,
        _config: &Config,
        _session: &Session,
    ) -> TransientBackend<R> {
        TransientBackend::new()
    }
}
