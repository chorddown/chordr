use serde::{Deserialize, Serialize};

/// The `CommandType` describes the action to perform
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandType {
    Add,
    Update,
    Upsert,
    Delete,
}
