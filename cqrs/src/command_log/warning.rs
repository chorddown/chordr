use crate::command::CommandConflictType;
use crate::record_trait::RecordTrait;

#[derive(Debug, PartialEq)]
pub struct Warning<R: RecordTrait> {
    pub conflict_type: Option<CommandConflictType>,
    pub sequence_number: usize,
    pub record_id: R::Id,
}
