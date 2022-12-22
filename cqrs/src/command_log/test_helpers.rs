use super::*;
use crate::command::CommandConflictType;
use crate::nonblocking::CommandExecutor;
use crate::prelude::{Command, RecordTrait};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::{RwLock, RwLockWriteGuard};

pub fn upsert(sequence_number: usize, id: i32, value: usize) -> LogEntry<Data, ()> {
    LogEntry {
        sequence_number,
        command: Data { id, value }.into_upsert_command(),
    }
}

pub fn add(sequence_number: usize, id: i32, value: usize) -> LogEntry<Data, ()> {
    LogEntry {
        sequence_number,
        command: Data { id, value }.into_add_command(),
    }
}

pub fn update(sequence_number: usize, id: i32, value: usize) -> LogEntry<Data, ()> {
    LogEntry {
        sequence_number,
        command: Data { id, value }.into_update_command(),
    }
}

pub fn delete(sequence_number: usize, id: i32, value: usize) -> LogEntry<Data, ()> {
    LogEntry {
        sequence_number,
        command: Data { id, value }.into_delete_command(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub id: i32,
    pub value: usize,
}

impl Data {
    pub fn into_upsert_command(self) -> Command<Data, ()> {
        Command::upsert(self, ())
    }

    pub fn into_add_command(self) -> Command<Data, ()> {
        Command::add(self, ())
    }

    pub fn into_update_command(self) -> Command<Data, ()> {
        Command::update(self, ())
    }

    pub fn into_delete_command(self) -> Command<Data, ()> {
        Command::delete(self, ())
    }
}

impl RecordTrait for Data {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl LogEntryRecord for Data {}

impl LogEntryContext for () {}

#[derive(Debug, Default)]
pub struct SimpleCX {
    state: RwLock<HashMap<i32, Data>>,
}

impl SimpleCX {
    pub fn new(initial_data: &[Data]) -> Self {
        let mut hash_map = HashMap::new();
        for initial_datum in initial_data {
            hash_map.insert(initial_datum.id, initial_datum.clone());
        }

        Self {
            state: RwLock::new(hash_map),
        }
    }
    pub fn sum(&self) -> usize {
        self.state
            .read()
            .unwrap()
            .iter()
            .fold(0, |acc, (_, d)| acc + d.value)
    }

    fn get_state_for_writing(&self) -> RwLockWriteGuard<HashMap<i32, Data>> {
        self.state.write().unwrap()
    }
}

#[derive(Debug)]
pub struct SimpleError {
    msg: String,
    command_error_type: Option<CommandConflictType>,
}

impl CommandConflictTrait for SimpleError {
    fn command_conflict_type(&self) -> Option<CommandConflictType> {
        self.command_error_type
    }
}

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl Error for SimpleError {}

#[async_trait(? Send)]
impl CommandExecutor for SimpleCX {
    type RecordType = Data;
    type Error = SimpleError;
    type Context = ();

    async fn upsert(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        let mut state = self.get_state_for_writing();
        let data = command.record();
        state.insert(data.id, data.clone());
        Ok(())
    }

    async fn add(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        let mut state = self.get_state_for_writing();
        let data = command.record();
        if state.get(&data.id).is_some() {
            Err(SimpleError {
                msg: format!("ID {} already exists", data.id),
                command_error_type: Some(CommandConflictType::RecordExists),
            })
        } else {
            let data = command.record();
            state.insert(data.id, data.clone());
            Ok(())
        }
    }

    async fn update(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        let mut state = self.get_state_for_writing();
        let data = command.record();
        if state.get(&data.id).is_none() {
            Err(SimpleError {
                msg: format!("ID {} does not exist", data.id),
                command_error_type: Some(CommandConflictType::RecordNotFound),
            })
        } else {
            let data = command.record();
            state.insert(data.id, data.clone());
            Ok(())
        }
    }

    async fn delete(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        let mut state = self.get_state_for_writing();
        let data = command.record();
        if state.get(&data.id).is_none() {
            Err(SimpleError {
                msg: format!("ID {} does not exist", data.id),
                command_error_type: Some(CommandConflictType::RecordNotFound),
            })
        } else {
            let data = command.record();
            state.remove(&data.id);
            Ok(())
        }
    }
}
