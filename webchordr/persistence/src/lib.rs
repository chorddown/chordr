use errors::WebError;
use webchordr_common::config;
use webchordr_common::constants;
use webchordr_common::errors;
use webchordr_common::fetch_helper;
use webchordr_common::helpers;
use webchordr_common::lock;

#[deprecated(note = "Use V2 Backend")]
pub mod backend;
pub mod backend_v2;
pub mod browser_storage;
pub mod command_context;
#[deprecated(note = "Use V2 Backend")]
pub mod persistence_manager;
pub mod session;
mod shared;
pub mod storage_key_utility;
#[doc(hidden)]
#[cfg(test)]
mod test_helpers;
pub mod web_repository;

pub mod prelude {
    #[allow(deprecated)]
    pub use super::backend::BackendTrait;
    pub use super::browser_storage::BrowserStorage;
    pub use super::command_context::CommandContext;
    #[allow(deprecated)]
    pub use super::persistence_manager::{PersistenceManager, PersistenceManagerTrait};
    pub use super::web_repository::{SetlistWebRepository, WebRepositoryTrait};
}
