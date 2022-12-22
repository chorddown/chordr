pub use self::log_entry::{LogEntry, LogEntryContext, LogEntryRecord};
use self::warning::Warning;
use crate::command::CommandConflictTrait;
use crate::command_log::conflict_resolver::ConflictResolver;
use crate::nonblocking::CommandExecutor;
use std::fmt::Debug;

mod conflict_resolver;
mod log_entry;
mod warning;

/// Implementation of a Replicated State Machine for synchronizing Chordr data
/// TODO: Prevent logging during the log processing
pub struct Rsm<
    'a,
    R: LogEntryRecord,
    C: LogEntryContext,
    E: CommandConflictTrait,
    CX: CommandExecutor<RecordType = R, Context = C, Error = E>,
> {
    command_executor: &'a CX,
}

impl<
        'a,
        R: LogEntryRecord,
        C: LogEntryContext,
        E: CommandConflictTrait,
        CX: CommandExecutor<RecordType = R, Context = C, Error = E>,
    > Rsm<'a, R, C, E, CX>
{
    pub async fn process_log_entries(
        &self,
        entries: &[LogEntry<R, C>],
    ) -> Result<Vec<Warning<R>>, E>
    where
        R::Id: Debug,
    {
        let mut warnings = vec![];

        for entry in entries {
            println!("Process entry {:?}", entry);
            let warning_option = self.process_log_entry(entry).await?;
            if let Some(warning) = warning_option {
                warnings.push(warning);
            }
        }
        Ok(warnings)
    }

    async fn process_log_entry(&self, entry: &LogEntry<R, C>) -> Result<Option<Warning<R>>, E>
    where
        R::Id: Debug,
    {
        if let Err(e) = self.command_executor.perform(&entry.command).await {
            return match e.command_conflict_type() {
                Some(conflict_type) => {
                    let conflict_resolver = ConflictResolver::default();
                    conflict_resolver
                        .resolve(conflict_type, self.command_executor, entry)
                        .await
                        .map(Some)
                        .map_err(|_| e)
                }
                // We can not recover from an un-categorized error
                None => Err(e),
            };
        }

        Ok(None)
    }
}

#[cfg(test)]
mod test_helpers;

#[cfg(test)]
mod tests {
    use crate::command::CommandConflictType;

    use super::test_helpers::*;
    use super::*;

    #[tokio::test]
    async fn process_log_entries_on_fresh_storage_test() {
        let cx = SimpleCX::default();
        let rsm = Rsm {
            command_executor: &cx,
        };
        let entries = [
            add(0, 0, 2),
            add(1, 1, 4),
            update(2, 1, 6),
            delete(3, 0, 2),
            upsert(4, 2, 4),
            upsert(5, 2, 6),
        ];

        let result = rsm.process_log_entries(&entries).await;
        assert!(result.is_ok());
        assert_eq!(12, cx.sum());
    }

    #[tokio::test]
    async fn process_log_entries_with_conflict_test() {
        let cx = SimpleCX::new(&[Data { id: 1, value: 0 }]);
        let rsm = Rsm {
            command_executor: &cx,
        };
        let entries = [
            add(0, 0, 2),
            add(1, 1, 4),    // This conflict will be resolved by deleting the old entry
            update(2, 2, 6), // This conflict can not be resolved
        ];

        let result = rsm.process_log_entries(&entries).await;
        assert!(result.is_err());
        assert_eq!("ID 2 does not exist", &result.unwrap_err().to_string());
    }

    #[tokio::test]
    async fn recover_from_record_exists_with_add_test() {
        let cx = SimpleCX::new(&[Data { id: 1, value: 0 }]);
        let rsm = Rsm {
            command_executor: &cx,
        };
        let entries = [
            add(0, 0, 2),
            add(1, 1, 4), // This conflict will be resolved by deleting the old entry
            add(2, 2, 6),
            delete(3, 2, 2),
            delete(4, 0, 2),
        ];

        let result = rsm.process_log_entries(&entries).await;
        assert!(result.is_ok());
        let warnings = result.unwrap();
        assert_eq!(
            warnings.len(),
            1,
            "Processed with {} warnings",
            warnings.len()
        );
        assert_eq!(4, cx.sum());
    }

    #[tokio::test]
    async fn recover_from_record_exists_with_update_test() {
        let cx = SimpleCX::default();
        let rsm = Rsm {
            command_executor: &cx,
        };
        let entries = [update(0, 4, 2)];

        let result = rsm.process_log_entries(&entries).await;
        assert!(result.is_err());
        assert_eq!(
            Some(CommandConflictType::RecordNotFound),
            result.unwrap_err().command_conflict_type()
        );
    }

    #[tokio::test]
    async fn recover_from_record_exists_with_delete_test() {
        let cx = SimpleCX::default();
        let rsm = Rsm {
            command_executor: &cx,
        };
        let entries = [
            delete(0, 1, 2), // This Delete Command actually is a conflict, but since the record does not exist the resulting state is still valid
        ];

        let result = rsm.process_log_entries(&entries).await;
        assert!(result.is_ok());
        let warnings = result.unwrap();
        assert_eq!(
            warnings.len(),
            1,
            "Processed with {} warnings",
            warnings.len()
        );
        assert_eq!(0, cx.sum());
    }
}
