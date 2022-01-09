use async_trait::async_trait;

use libchordr::prelude::RecordTrait;
use webchordr_common::tri::Tri;

use crate::persistence_manager::CommandContext;
use crate::WebError;

/// Web Repository provides the functions to manage the persistence of a specific type
///
/// In addition to the methods defined in the trait, the async functions `store` and `load`
/// must be implemented
#[async_trait(? Send)]
pub trait WebRepositoryTrait {
    type ManagedType: RecordTrait;

    /// Return the `namespace` part of the storage key
    fn namespace() -> &'static str;

    /// Return the `key` part of the storage key
    fn key() -> &'static str;

    /// Return the context instance for this repository
    fn build_context() -> CommandContext;

    /// Store the given `value`
    async fn store(&mut self, value: &Self::ManagedType) -> Result<(), WebError>;

    /// Load the stored value
    async fn load(&mut self) -> Tri<Self::ManagedType, WebError>;
}
