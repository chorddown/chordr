pub use command_executor::CommandExecutor;
pub use command_type::CommandType;

use crate::RecordTrait;

mod command_executor;
mod command_type;

/// A `Command` defines an operation to mutate data in the system
/// It is defined by a [`CommandType`] describing the action to perform and the subject of the
/// operation
pub struct Command<T: RecordTrait, C> {
    command_type: CommandType,
    subject: T,
    context: C,
}

impl<T: RecordTrait, C> Command<T, C> {
    /// Create a new command to `Upsert` `record` to the system
    pub fn upsert(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Upsert,
            subject: record,
            context,
        }
    }

    /// Create a new command to `Add` `record` to the system
    pub fn add(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Add,
            subject: record,
            context,
        }
    }

    /// Create a new command to `Update` `record` in the system
    pub fn update(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Update,
            subject: record,
            context,
        }
    }

    /// Create a new command to `Delete` the record with `id` from the system
    pub fn delete(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Delete,
            subject: record,
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
    pub fn record(&self) -> &T {
        &self.subject
    }

    /// Return the `Command`'s context
    pub fn context(&self) -> &C {
        &self.context
    }
}
