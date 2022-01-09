pub use command_executor::CommandExecutor;
pub use command_type::CommandType;

use crate::RecordTrait;

mod command_executor;
mod command_type;

enum Subject<T: RecordTrait> {
    Record(T),
    Id(T::Id),
}

/// A `Command` defines an operation to mutate data in the system
/// It is defined by a [`CommandType`] describing the action to perform and the subject of the
/// operation
pub struct Command<T: RecordTrait, C> {
    command_type: CommandType,
    subject: Subject<T>,
    context: C,
}

impl<T: RecordTrait, C> Command<T, C> {
    /// Create a new command to `Add` `record` to the system
    pub fn add(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Add,
            subject: Subject::Record(record),
            context,
        }
    }

    /// Create a new command to `Update` `record` in the system
    pub fn update(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Update,
            subject: Subject::Record(record),
            context,
        }
    }

    /// Create a new command to `Delete` the record with `id` from the system
    pub fn delete(id: T::Id, context: C) -> Self {
        Self {
            command_type: CommandType::Delete,
            subject: Subject::Id(id),
            context,
        }
    }

    /// Return the `Command`'s type
    pub fn command_type(&self) -> CommandType {
        self.command_type
    }

    /// Return the `Command`'s record
    ///
    /// Will return a reference to the record to be added/updated in the system
    /// This is applicable for `Add` and `Update` `Command`s
    pub fn record(&self) -> Option<&T> {
        match self.subject {
            Subject::Record(ref r) => Some(r),
            Subject::Id(_) => None,
        }
    }

    /// Return the `Command`'s subject ID
    ///
    /// Will return a reference to the ID to be removed from the system
    /// This is applicable for `Delete` `Command`s
    pub fn id(&self) -> Option<&T::Id> {
        match self.subject {
            Subject::Record(_) => None,
            Subject::Id(ref r) => Some(r),
        }
    }

    /// Return the `Command`'s context
    pub fn context(&self) -> &C {
        &self.context
    }
}