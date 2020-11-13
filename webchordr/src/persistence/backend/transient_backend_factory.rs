use crate::config::Config;
use crate::persistence::backend::TransientBackend;
use crate::session::Session;

pub struct TransientBackendFactory {}

impl TransientBackendFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, _config: &Config, _session: &Session) -> TransientBackend {
        TransientBackend::new()
    }
}
