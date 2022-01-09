use async_trait::async_trait;

use tri::Tri;

use crate::count::Count;
use crate::RecordTrait;

#[async_trait(?Send)]
pub trait AsyncRepositoryTrait {
    type ManagedType: RecordTrait;
    type Error;

    /// Find all instances of `ManagedType` in the `Repository`
    async fn find_all(&self) -> Result<Vec<Self::ManagedType>, Self::Error>;

    /// Count all instances of `ManagedType` in the `Repository`
    async fn count_all(&self) -> Result<Count, Self::Error>;

    /// Find an instance of `ManagedType` with `id` inside the `Repository`
    async fn find_by_id(
        &self,
        id: <Self::ManagedType as RecordTrait>::Id,
    ) -> Tri<Self::ManagedType, Self::Error>;

    /// Add the instance of `ManagedType` to the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    async fn add(&self, instance: Self::ManagedType) -> Result<(), Self::Error>;

    /// Update the matching instance of `ManagedType` inside the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    async fn update(&self, instance: Self::ManagedType) -> Result<(), Self::Error>;

    /// Delete the matching instance of `ManagedType` from the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    async fn delete(&self, instance: Self::ManagedType) -> Result<(), Self::Error>;
}
