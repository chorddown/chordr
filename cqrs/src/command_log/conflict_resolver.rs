use super::{
    log_entry::{LogEntry, LogEntryContext, LogEntryRecord},
    warning::Warning,
};
use crate::command::{Command, CommandConflictType, CommandType};
use crate::{command::CommandConflictTrait, nonblocking::CommandExecutor};
use std::fmt::Debug;
use CommandConflictType::RecordExists;
use CommandConflictType::RecordNotFound;

#[derive(Default)]
pub struct ConflictResolver {}

impl ConflictResolver {
    pub async fn resolve<
        R: LogEntryRecord,
        C: LogEntryContext,
        E: CommandConflictTrait,
        CX: CommandExecutor<RecordType = R, Context = C, Error = E>,
    >(
        &self,
        conflict_type: CommandConflictType,
        command_executor: &CX,
        entry: &LogEntry<R, C>,
    ) -> Result<Warning<R>, ()>
    where
        R::Id: Debug,
    {
        let did_resolve_conflict = match conflict_type {
            RecordNotFound => self.resolve_record_not_found(entry).is_ok(),
            RecordExists => self
                .resolve_record_exists(command_executor, entry)
                .await
                .is_ok(),
        };
        if did_resolve_conflict {
            let warning = Warning {
                conflict_type: Some(conflict_type),
                sequence_number: entry.sequence_number,
                record_id: entry.record_id(),
            };
            dbg!("Warning: {:?}", &warning);
            Ok(warning)
        } else {
            // We could not recover from the error => stop the log execution
            Err(())
        }
    }

    fn resolve_record_not_found<R: LogEntryRecord, C: LogEntryContext>(
        &self,
        entry: &LogEntry<R, C>,
    ) -> Result<(), ()> {
        match entry.command.command_type {
            // Update Command for non-existing records will fail
            CommandType::Update => Err(()),
            // If the original Command was a Delete command, nothing needs to be done
            CommandType::Delete => Ok(()),
            CommandType::Add => unreachable!(),
            CommandType::Upsert => unreachable!(),
        }
    }

    async fn resolve_record_exists<
        R: LogEntryRecord,
        C: LogEntryContext,
        E: CommandConflictTrait,
        CX: CommandExecutor<RecordType = R, Context = C, Error = E>,
    >(
        &self,
        command_executor: &CX,
        entry: &LogEntry<R, C>,
    ) -> Result<(), E> {
        let Command {
            record, context, ..
        } = entry.command.clone();

        // Delete the offending entry
        command_executor
            .delete(&Command::delete(record, context))
            .await?;

        // Try to perform the original Command again
        command_executor.perform(&entry.command).await
    }
}
