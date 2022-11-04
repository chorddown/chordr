use crate::backend_v2::server_backend::ServerBackend;
use crate::config::Config;
use cqrs::prelude::RecordTrait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use webchordr_common::session::{Session, SessionUser};

#[derive(Default)]
pub struct ServerBackendFactory {}

impl ServerBackendFactory {
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
    /// use webchordr_persistence::backend_v2::server_backend_factory::ServerBackendFactory;
    /// use webchordr_persistence::backend_v2::server_backend::ServerBackend;
    /// let _storage: ServerBackend<Value> =
    ///             ServerBackendFactory::default().build(&Config::default(), &Session::default());
    /// ```
    pub fn build<R: RecordTrait + Serialize + DeserializeOwned>(
        &self,
        config: &Config,
        session: &Session,
    ) -> ServerBackend<R> {
        let credentials = match session.user() {
            SessionUser::Unauthenticated => None,
            SessionUser::LoggedIn(user) => Some(user.into()),
        };

        ServerBackend::new(config.api_url().to_owned(), credentials)
    }
}

#[cfg(test)]
mod test {
    use crate::backend_v2::server_backend::ServerBackend;
    use crate::backend_v2::server_backend_factory::ServerBackendFactory;
    use crate::test_helpers::TestValue;
    use webchordr_common::config::Config;
    use webchordr_common::session::Session;

    #[test]
    fn build() {
        let _storage: ServerBackend<TestValue> =
            ServerBackendFactory::default().build(&Config::default(), &Session::default());
    }
}
