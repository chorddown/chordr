/// This file was auto-generated by cqrs-desync on 2022-10-23 18:21:20
/// Do not edit it

use crate::count::Count;
use crate::RecordTrait;

use tri::Tri;


pub trait RepositoryTrait {
    type ManagedType: RecordTrait;
    type Error;

    /// Find all instances of `ManagedType` in the `Repository`
    fn find_all(&self) -> Result<Vec<Self::ManagedType>, Self::Error>;

    /// Count all instances of `ManagedType` in the `Repository`
    fn count_all(&self) -> Result<Count, Self::Error>;

    /// Find an instance of `ManagedType` with `id` inside the `Repository`
    fn find_by_id(
        &self,
        id: <Self::ManagedType as RecordTrait>::Id,
    ) -> Tri<Self::ManagedType, Self::Error>;

    /// Save the instance of `ManagedType` to the `Repository`
    ///
    /// If a record with the instance's ID (= `Self::ManagedType as RecordTrait>::Id`) already
    /// exists in the Repository, it's value will be replaced by the given instance's value.
    /// If no such record exists, it will be added to the Repository as a new entry
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn save(&self, instance: Self::ManagedType) -> Result<(), Self::Error>;

    /// Add the instance of `ManagedType` to the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn add(&self, instance: Self::ManagedType) -> Result<(), Self::Error>;

    /// Update the matching instance of `ManagedType` inside the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn update(&self, instance: Self::ManagedType) -> Result<(), Self::Error>;

    /// Delete the matching instance of `ManagedType` from the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn delete(&self, instance: Self::ManagedType) -> Result<(), Self::Error>;
}
