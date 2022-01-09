use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use cqrs::prelude::{Command, RecordTrait};
use webchordr_common::errors::{PersistenceError, WebError};

use crate::persistence_manager::CommandContext;
use crate::storage_key_utility::build_combined_id_key;
use webchordr_common::tri::Tri;

#[derive(Debug)]
pub(crate) enum ExistenceCheck {
    MustExist,
    MustNotExist,
}

/// Add or Update the data according to the given command
///
/// The Add and Update commands are basically only different whether the record should already exist
/// in the database
pub(crate) fn store_with_command<T: Serialize + RecordTrait, EC, SC>(
    command: &Command<T, CommandContext>,
    existence_check: ExistenceCheck,
    exists_callback: EC,
    store_callback: SC,
) -> Result<(), WebError>
where
    EC: Fn(&str) -> bool,
    SC: Fn(String, String) -> Result<(), WebError>,
{
    let record = match command.record() {
        None => return Err(PersistenceError::general_error("No command record given").into()),
        Some(r) => r,
    };
    let combined_id_key = build_combined_id_key::<T>(&command.context(), &record.id());
    let serialized_value = serde_json::to_string(record)?;
    let entry_does_exist = exists_callback(&combined_id_key);
    match existence_check {
        ExistenceCheck::MustExist if !entry_does_exist => {
            return Err(record_not_found_error());
        }
        ExistenceCheck::MustNotExist if entry_does_exist => {
            return Err(record_exists_error());
        }
        _ => {}
    }

    store_callback(combined_id_key, serialized_value)
}

pub(crate) fn hash_map_from_context_and_slice<T: Serialize + RecordTrait>(
    context: &CommandContext,
    entries: &[T],
) -> HashMap<String, String> {
    let mut map = HashMap::with_capacity(entries.len());
    for entry in entries {
        map.insert(
            build_combined_id_key::<T>(&context, &entry.id()),
            serde_json::to_string(&entry).unwrap(),
        );
    }
    map
}

pub(crate) fn deserialize_value<T>(serialized: &str) -> Tri<T, WebError>
where
    T: for<'a> Deserialize<'a>,
{
    match serde_json::from_str(serialized) {
        Ok(deserialized) => Tri::from_option(deserialized),
        Err(e) => Tri::Err(
            PersistenceError::deserialization_error(e, Some(serialized.to_string())).into(),
        ),
    }
}

pub(crate) fn record_not_found_error() -> WebError {
    return PersistenceError::general_error("A record with the given ID does not exist").into();
}

pub(crate) fn record_exists_error() -> WebError {
    return PersistenceError::general_error("A record with the given ID already exists").into();
}

pub(crate) fn missing_record_id_error() -> WebError {
    return PersistenceError::general_error("No ID given").into();
}
