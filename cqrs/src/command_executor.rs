use crate::command::Command;
use crate::command_type::CommandType;
use libchordr::models::record_trait::RecordTrait;

pub trait CommandExecutor {
    type RecordType: RecordTrait;
    type Error;

    fn perform(&self, command: Command<Self::RecordType>) -> Result<(), Self::Error> {
        match command.command_type() {
            CommandType::Add => self.add(command),
            CommandType::Update => self.update(command),
            CommandType::Delete => self.delete(command),
        }
    }

    fn add(&self, command: Command<Self::RecordType>) -> Result<(), Self::Error>;
    fn update(&self, command: Command<Self::RecordType>) -> Result<(), Self::Error>;
    fn delete(&self, command: Command<Self::RecordType>) -> Result<(), Self::Error>;
}
