use std::hash::Hash;

/// Trait for structs that have a unique identifier
///
/// See also http://docs.diesel.rs/diesel/associations/trait.Identifiable.html
pub trait RecordTrait {
    /// The type of this structs identifier
    ///
    /// For single-field primary keys, this is typically `&'a i32`, or `&'a String`
    /// For composite primary keys, this is typically `(&'a i32, &'a i32)`
    /// or `(&'a String, &'a String)`, etc.
    type Id: Hash + Eq;

    /// Returns the identifier for this record.
    ///
    /// This takes `self` by value, not reference.
    /// This is because composite primary keys
    /// are typically stored as multiple fields.
    /// We could not return `&(String, String)` if each string is a separate field.
    ///
    /// Because of Rust's rules about specifying lifetimes,
    /// this means that `RecordTrait` is usually implemented on references
    /// so that we have a lifetime to use for `Id`.
    fn id(&self) -> Self::Id;
}

impl RecordTrait for i32 {
    type Id = i32;

    fn id(&self) -> Self::Id {
        *self
    }
}

// impl<T: RecordTrait> RecordTrait for &std::rc::Rc<T> {
//     type Id = <T as RecordTrait>::Id;
//
//     fn id(self) -> Self::Id {
//         &self.as_ref().id()
//     }
// }
