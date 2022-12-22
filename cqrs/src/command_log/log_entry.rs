use crate::prelude::{Command, RecordTrait};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// pub trait LogEntryRecord: RecordTrait + Serialize + DeserializeOwned {}
// pub trait LogEntryContext: Serialize + DeserializeOwned {}
pub trait LogEntryRecord: RecordTrait + Clone + Debug {}
pub trait LogEntryContext: Clone + Debug {}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry<T: LogEntryRecord, C: LogEntryContext> {
    pub command: Command<T, C>,
    pub sequence_number: usize,
}

impl<T: LogEntryRecord, C: LogEntryContext> LogEntry<T, C> {
    pub fn record_id(&self) -> T::Id {
        self.command.record().id()
    }
}
