use webchordr_common::session::Session;

use crate::backend::TransientBackend;
use crate::config::Config;

pub struct TransientBackendFactory {}

impl TransientBackendFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, _config: &Config, _session: &Session) -> TransientBackend {
        TransientBackend::new()
    }
}

impl Default for TransientBackendFactory {
    fn default() -> Self {
        Self::new()
    }
}
