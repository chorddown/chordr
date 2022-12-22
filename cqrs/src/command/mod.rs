use crate::RecordTrait;
pub use command_conflict::*;
#[deprecated(note = "Use either blocking or unblocking API")]
pub use command_executor::CommandExecutor;
pub use command_type::CommandType;
use serde::{Deserialize, Serialize};

mod command_conflict;
mod command_executor;
mod command_type;

// pub trait CommandTrait {
//     type RecordType: RecordTrait;
//     type Context;
//
//     /// Return the `Command`'s type
//     fn command_type(&self) -> CommandType;
//
//     /// Return the `Command`'s record
//     ///
//     /// Will return a reference to the record to be added/updated in the system
//     /// This is applicable for `Add` and `Update` `Command`s
//     fn record(&self) -> &Self::RecordType;
//
//     /// Return the `Command`'s context
//     fn context(&self) -> &Self::Context;
// }

/// A `Command` defines an operation to mutate data in the system
/// It is defined by a [`CommandType`] describing the action to perform and the subject of the
/// operation
#[derive(Debug, Serialize, Deserialize)]
pub struct Command<T: RecordTrait, C> {
    pub(crate) command_type: CommandType,
    pub(crate) record: T,
    pub(crate) context: C,
}

impl<T: RecordTrait, C> Command<T, C> {
    /// Create a new command to `Upsert` `record` to the system
    pub fn upsert(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Upsert,
            record,
            context,
        }
    }

    /// Create a new command to `Add` `record` to the system
    pub fn add(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Add,
            record,
            context,
        }
    }

    /// Create a new command to `Update` `record` in the system
    pub fn update(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Update,
            record,
            context,
        }
    }

    /// Create a new command to `Delete` the record with `id` from the system
    pub fn delete(record: T, context: C) -> Self {
        Self {
            command_type: CommandType::Delete,
            record,
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
        &self.record
    }

    /// Return the `Command`'s context
    pub fn context(&self) -> &C {
        &self.context
    }
}

impl<T, C> Clone for Command<T, C>
where
    T: RecordTrait + Clone,
    C: Clone,
{
    fn clone(&self) -> Self {
        Self {
            command_type: self.command_type,
            record: self.record.clone(),
            context: self.context.clone(),
        }
    }
}
