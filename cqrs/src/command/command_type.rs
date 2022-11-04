use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The `CommandType` describes the action to perform
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandType {
    Add,
    Update,
    Upsert,
    Delete,
}

impl Display for CommandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CommandType::Add => "Add",
            CommandType::Update => "Update",
            CommandType::Upsert => "Upsert",
            CommandType::Delete => "Delete",
        })
    }
}
