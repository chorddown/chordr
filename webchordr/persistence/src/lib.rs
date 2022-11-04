use errors::WebError;
use webchordr_common::config;
use webchordr_common::constants;
use webchordr_common::errors;
use webchordr_common::fetch_helper;
use webchordr_common::helpers;

pub mod backend_v2;
pub mod browser_storage;
pub mod command_context;
pub mod session;
mod shared;
pub mod storage_key_utility;
#[doc(hidden)]
#[cfg(test)]
mod test_helpers;
pub mod web_repository;

pub mod prelude {
    pub use super::browser_storage::BrowserStorage;
    pub use super::command_context::CommandContext;
    pub use super::web_repository::{SetlistWebRepository, WebRepositoryTrait};
}
