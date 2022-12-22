pub trait CommandConflictTrait {
    fn command_conflict_type(&self) -> Option<CommandConflictType>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandConflictType {
    RecordExists,
    RecordNotFound,
}
