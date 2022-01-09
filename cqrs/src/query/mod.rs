pub use crate::query::query_type::QueryType;
use crate::RecordTrait;

mod query_type;

enum Subject<T: RecordTrait> {
    // Record(T),
    Id(T::Id),
    None,
}

/// A `Query` defines an operation to read data from the system
/// It is defined by a [`QueryType`] describing the type of search to perform and the subject of the
/// operation
pub struct Query<T: RecordTrait, C> {
    query_type: QueryType,
    subject: Subject<T>,
    context: C,
}

impl<T: RecordTrait, C> Query<T, C> {
    /// Create a new query to find all records
    pub fn all(context: C) -> Self {
        Self {
            query_type: QueryType::All,
            subject: Subject::None,
            context,
        }
    }

    /// Create a new query to find the record matching the `id`
    pub fn by_id(id: T::Id, context: C) -> Self {
        Self {
            query_type: QueryType::ById,
            subject: Subject::Id(id),
            context,
        }
    }

    /// Return the `Query`'s type
    pub fn query_type(&self) -> QueryType {
        self.query_type
    }

    // /// Return the `Query`'s record
    // ///
    // /// Will return a reference to the record to be added/updated in the system
    // /// This is applicable for `Add` and `Update` `Command`s
    // pub fn record(&self) -> Option<&T> {
    //     match self.subject {
    //         Subject::Record(ref r) => Some(r),
    //         Subject::Id(_) => None,
    //         Subject::None => None,
    //     }
    // }

    /// Return the `Query`'s subject ID
    ///
    /// Will return a reference to the ID to find
    /// This is applicable for `ById` `Query`s
    pub fn id(&self) -> Option<&T::Id> {
        match self.subject {
            // Subject::Record(_) => None,
            Subject::Id(ref r) => Some(r),
            Subject::None => None,
        }
    }

    /// Return the `Query`'s context
    pub fn context(&self) -> &C {
        &self.context
    }
}
