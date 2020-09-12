use crate::persistence::backend::TransientBackend;

pub struct TransientBackendFactory {}

impl TransientBackendFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> TransientBackend {
        TransientBackend::new()
    }
}
