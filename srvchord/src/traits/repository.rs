use crate::ConnectionType;
use libchordr::prelude::RecordTrait;

pub type Count = i64;

pub trait RepositoryTrait {
    type ManagedType: RecordTrait;
    type Error;

    /// Find all instances of `ManagedType` in the `Repository`
    fn find_all(&self, connection: &ConnectionType) -> Result<Vec<Self::ManagedType>, Self::Error>;

    /// Count all instances of `ManagedType` in the `Repository`
    fn count_all(&self, connection: &ConnectionType) -> Result<Count, Self::Error>;

    /// Find an instance of `ManagedType` with `id` inside the `Repository`
    fn find_by_id(
        &self,
        connection: &ConnectionType,
        id: <Self::ManagedType as RecordTrait>::Id,
    ) -> Result<Self::ManagedType, Self::Error>;

    /// Add the instance of `ManagedType` to the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn add(
        &self,
        connection: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error>;

    /// Update the matching instance of `ManagedType` inside the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn update(
        &self,
        connection: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error>;

    /// Delete the matching instance of `ManagedType` from the `Repository`
    ///
    /// # Errors
    ///
    /// This function will return an error if the database operation fails
    fn delete(
        &self,
        connection: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error>;
}

// # Success result
//
// This function will return a prepared instance of `object` (e.g. with values from
// auto-increment fields)
