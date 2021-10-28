use crate::backend::{
    BrowserStorageBackend, BrowserStorageBackendFactory, ServerBackend, ServerBackendFactory,
    TransientBackend, TransientBackendFactory,
};
use crate::browser_storage::BrowserStorage;
use crate::config::Config;
use crate::persistence_manager::PersistenceManager;
use webchordr_common::session::Session;

pub type PMType =
    PersistenceManager<BrowserStorageBackend<BrowserStorage>, ServerBackend, TransientBackend>;

pub struct PersistenceManagerFactory {}

impl PersistenceManagerFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, config: &Config, session: Session) -> PMType {
        let browser_storage_backend_factory = BrowserStorageBackendFactory::new();
        let server_backend_factory = ServerBackendFactory::new();
        let transient_backend_factory = TransientBackendFactory::new();

        let browser_storage_backend = browser_storage_backend_factory.build(config, &session);
        let server_backend = server_backend_factory.build(config, &session);
        let transient_backend = transient_backend_factory.build(config, &session);

        PersistenceManager::new(
            session,
            browser_storage_backend,
            server_backend,
            transient_backend,
        )
    }
}
