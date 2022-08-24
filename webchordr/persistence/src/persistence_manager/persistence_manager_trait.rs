use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use cqrs::prelude::{Count, RecordTrait};
use webchordr_common::errors::WebError;
use webchordr_common::tri::Tri;

use crate::persistence_manager::CommandContext;

/// The Persistence Manager will take care of storing and loading data.
///
/// The manager may use different backends for storing and loading data
/// and is responsible for synchronization of those.
#[async_trait(? Send)]
pub trait PersistenceManagerTrait // : BackendTrait
{
    /// Find all instances within the given Context
    async fn find_all<T>(&self, context: CommandContext) -> Result<Vec<T>, WebError>
    where
        T: for<'a> Deserialize<'a> + RecordTrait;

    /// Count all instances within the given Context
    async fn count_all(&self, context: CommandContext) -> Result<Count, WebError>;

    /// Find an instance with `id` within the given Context
    async fn find_by_id<T>(&self, context: CommandContext, id: T::Id) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a> + RecordTrait;

    /// Add the instance to the database (within the area of the given Context)
    ///
    /// # Errors
    ///
    /// This function will return an error if the storage operation fails
    async fn add<'a, T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &'a T,
    ) -> Result<(), WebError>
    where
        &'a T: RecordTrait;

    /// Update the matching instance in the database (within the area of the given Context)
    ///
    /// # Errors
    ///
    /// This function will return an error if the storage operation fails
    async fn update<'a, T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &'a T,
    ) -> Result<(), WebError>
    where
        &'a T: RecordTrait;

    /// Delete the matching instance from the database (within the area of the given Context)
    ///
    /// # Errors
    ///
    /// This function will return an error if the storage operation fails
    async fn delete<'a, T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &'a T,
    ) -> Result<(), WebError>
    where
        &'a T: RecordTrait;
}
