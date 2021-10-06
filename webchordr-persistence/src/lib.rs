use errors::WebError;
use webchordr_common::config;
use webchordr_common::constants;
use webchordr_common::errors;
use webchordr_common::fetch_helper;
use webchordr_common::helpers;
use webchordr_common::lock;

pub mod backend;
pub mod browser_storage;
pub mod persistence_manager;
pub mod session;
pub mod storage_key_utility;
pub mod web_repository;

pub mod prelude {
    pub use super::backend::BackendTrait;
    pub use super::browser_storage::BrowserStorage;
    pub use super::persistence_manager::{PersistenceManager, PersistenceManagerTrait};
    pub use super::web_repository::{SetlistWebRepository, WebRepositoryTrait};
}
