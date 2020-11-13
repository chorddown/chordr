use crate::config::Config;
use crate::persistence::backend::ServerBackend;
use crate::session::{Session, SessionUser};

pub struct ServerBackendFactory {}

impl ServerBackendFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, config: &Config, session: &Session) -> ServerBackend {
        let credentials = match session.user() {
            SessionUser::Unauthenticated => None,
            SessionUser::LoggedIn(user) => Some(user.into()),
        };

        ServerBackend::new(config.api_url().to_owned(), credentials)
    }
}
