use serde::{Deserialize, Serialize};

/// The `CommandType` describes the action to perform
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CommandType {
    Add,
    Update,
    Delete,
}
