use tri::Tri;

use crate::count::Count;
use crate::RecordTrait;

pub trait RepositoryTrait {
    type ManagedType: RecordTrait;
    type Error;
    type Context;

    /// Find all instances of `ManagedType` in the `Repository`
    fn find_all(&self, context: &Self::Context) -> Result<Vec<Self::ManagedType>, Self::Error>;

    /// Count all instances of `ManagedType` in the `Repository`
    fn count_all(&self, context: &Self::Context) -> Result<Count, Self::Error>;

    /// Find an instance of `ManagedType` with `id` inside the `Repository`
    fn find_by_id(
        &self,
        context: &Self::Context,
        id: <Self::ManagedType as RecordTrait>::Id,
    ) -> Tri<Self::ManagedType, Self::Error>;

    /// Add the instance of `ManagedType` to the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn add(&self, context: &Self::Context, instance: Self::ManagedType) -> Result<(), Self::Error>;

    /// Update the matching instance of `ManagedType` inside the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn update(
        &self,
        context: &Self::Context,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error>;

    /// Delete the matching instance of `ManagedType` from the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn delete(
        &self,
        context: &Self::Context,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error>;
}
