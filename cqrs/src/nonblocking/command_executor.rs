use crate::command::{Command, CommandType};
use crate::RecordTrait;
use async_trait::async_trait;

#[async_trait(? Send)]
pub trait CommandExecutor {
    type RecordType: RecordTrait;
    type Error;
    type Context;

    async fn perform(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        match command.command_type() {
            CommandType::Upsert => self.upsert(command),
            CommandType::Add => self.add(command),
            CommandType::Update => self.update(command),
            CommandType::Delete => self.delete(command),
        }
        .await
    }

    /// Save the `record` to the system
    async fn upsert(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error>;

    /// Add the `record` to the system
    ///
    /// An error will be returned if the `record` already exists
    async fn add(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error>;

    /// Update the `record` in the system
    ///
    /// An error will be returned if the `record` does not exist
    async fn update(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error>;

    /// Delete the `record` from the system
    ///
    /// An error will be returned if the `record` does not exist
    async fn delete(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error>;
}
