use crate::persistence::backend::BackendTrait;
use async_trait::async_trait;

/// The Persistence Manager will take care of storing and loading data.
///
/// The manager may use different backends for storing and loading data
/// and is responsible for synchronization of those.
#[async_trait(? Send)]
pub trait PersistenceManagerTrait: BackendTrait {}
