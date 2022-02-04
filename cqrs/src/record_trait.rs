use std::fmt::Display;
use std::hash::Hash;

/// Trait for structs that have a unique identifier
///
/// See also http://docs.diesel.rs/diesel/associations/trait.Identifiable.html
pub trait RecordTrait {
    /// The type of this structs identifier
    type Id: Hash + Eq + Display;

    /// Return the identifier for this record
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
