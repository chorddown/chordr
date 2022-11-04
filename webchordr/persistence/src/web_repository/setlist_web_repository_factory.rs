use crate::backend_v2::browser_storage_backend::BrowserStorageBackend;
use crate::backend_v2::persistence_manager::PersistenceManagerV2;
use crate::backend_v2::server_backend::ServerBackend;
use crate::backend_v2::server_backend_factory::ServerBackendFactory;
use crate::browser_storage::BrowserStorage;
use crate::command_context::CommandContext;
use crate::web_repository::SetlistWebRepository;
use cqrs::nonblocking::{CommandExecutor, QueryExecutor};
use libchordr::prelude::Setlist;
use webchordr_common::config::Config;
use webchordr_common::prelude::WebError;
use webchordr_common::session::Session;

type CE = dyn CommandExecutor<Context = CommandContext, Error = WebError, RecordType = Setlist>;

type QE = dyn QueryExecutor<Context = CommandContext, Error = WebError, RecordType = Setlist>;

pub struct SetlistWebRepositoryFactory {}

impl SetlistWebRepositoryFactory {
    pub fn build(config: &Config, session: &Session) -> SetlistWebRepository {
        let persistence_manager = PersistenceManagerV2::with_backends(
            build_command_backends(config, session),
            build_query_backends(config, session),
        );
        SetlistWebRepository::new(persistence_manager)
    }
}

fn build_command_backends(config: &Config, session: &Session) -> Vec<Box<CE>> {
    let browser_storage_backend = build_browser_storage_backend();
    if session.is_authenticated() {
        let server_backend = build_server_backend(config, session);
        vec![browser_storage_backend, server_backend]
    } else {
        vec![browser_storage_backend]
    }
}

fn build_query_backends(config: &Config, session: &Session) -> Vec<Box<QE>> {
    let browser_storage_backend = build_browser_storage_backend();
    if session.is_authenticated() {
        let server_backend = build_server_backend(config, session);
        vec![server_backend, browser_storage_backend]
    } else {
        vec![browser_storage_backend]
    }
}

fn build_browser_storage_backend() -> Box<BrowserStorageBackend<BrowserStorage, Setlist>> {
    Box::new(BrowserStorageBackend::new(get_browser_storage()))
}

fn build_server_backend(config: &Config, session: &Session) -> Box<ServerBackend<Setlist>> {
    Box::new(ServerBackendFactory::new().build(config, session))
}

fn get_browser_storage() -> BrowserStorage {
    let browser_storage = BrowserStorage::local_storage().expect("Could not get browser storage");
    browser_storage
}
