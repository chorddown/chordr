use crate::backend_v2::browser_storage_backend::BrowserStorageBackend;
use crate::backend_v2::persistence_manager::PersistenceManagerV2;
use crate::browser_storage::BrowserStorage;
use crate::command_context::CommandContext;
use crate::web_repository::SettingsWebRepository;
use cqrs::nonblocking::{CommandExecutor, QueryExecutor};
use libchordr::prelude::SongSettingsMap;
use webchordr_common::errors::WebError;

pub struct SettingsWebRepositoryFactory {}

impl SettingsWebRepositoryFactory {
    pub fn build() -> SettingsWebRepository {
        let browser_storage =
            BrowserStorage::local_storage().expect("Could not get browser storage");

        type CE = Box<
            dyn CommandExecutor<
                Context = CommandContext,
                Error = WebError,
                RecordType = SongSettingsMap,
            >,
        >;
        let browser_storage_backend = Box::new(BrowserStorageBackend::new(browser_storage.clone()));
        let command_backends: Vec<CE> = vec![browser_storage_backend];

        type QE = Box<
            dyn QueryExecutor<
                Context = CommandContext,
                Error = WebError,
                RecordType = SongSettingsMap,
            >,
        >;
        let browser_storage_backend = Box::new(BrowserStorageBackend::new(browser_storage));
        let query_backends: Vec<QE> = vec![browser_storage_backend];

        let persistence_manager: PersistenceManagerV2<SongSettingsMap> =
            PersistenceManagerV2::with_backends(command_backends, query_backends);
        SettingsWebRepository::new(persistence_manager)
    }
}
