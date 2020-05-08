// pub trait InputTrait {
//     type Error;
//     type Target;
//     type AdditionalData;
//
//     fn transform(self) -> Result<Self::Target, Self::Error>;
// }

// pub trait DatabaseValueProvider {
//     type Output;
//
//     fn transform(self) -> Self::Output;
// }

// use diesel::Identifiable;
//
// pub trait RepositoryTrait {
//     type ManagedType: Identifiable;
//     type Error;
//
//     /// Find an instance of `ManagedType` with `id` inside the `Repository`
//     fn find_by_id(id: <Self::ManagedType as Identifiable>::Id) -> Result<Self::ManagedType, Self::Error>;
//
//     /// Add the instance of `ManagedType` to the `Repository`
//     ///
//     /// # Success result
//     ///
//     /// This function will return a prepared instance of `object` (e.g. with values from
//     /// auto-increment fields)
//     ///
//     ///
//     /// # Errors
//     ///
//     /// This function will return an error if the database operation fails
//     fn add(object: Self::ManagedType) -> Result<Self::ManagedType, Self::Error>;
//
//     /// Update the matching instance of `ManagedType` inside the `Repository`
//     ///
//     /// # Errors
//     ///
//     /// This function will return an error if the database operation fails
//     fn update(instance: Self::ManagedType) -> Result<(), Self::Error>;
//
//     /// Delete the matching instance of `ManagedType` from the `Repository`
//     ///
//     /// # Errors
//     ///
//     /// This function will return an error if the database operation fails
//     fn delete(instance: Self::ManagedType) -> Result<(), Self::Error>;
// }
