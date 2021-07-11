pub mod backend;
pub mod browser_storage;
pub mod persistence_manager;
pub mod storage_key_utility;
pub mod web_repository;

pub mod prelude {
    pub use super::backend::BackendTrait;
    pub use super::browser_storage::BrowserStorage;
    pub use super::persistence_manager::{PersistenceManager, PersistenceManagerTrait};
    pub use super::web_repository::{SetlistWebRepository, WebRepositoryTrait};
}
