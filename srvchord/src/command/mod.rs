use crate::ConnectionType;

#[deprecated(note = "Use cqrs::prelude::* instead")]
pub enum CommandType {
    Add,
    Update,
    Delete,
}

#[deprecated(note = "Use cqrs::prelude::* instead")]
#[allow(deprecated)]
pub struct Command<'a> {
    pub command_type: CommandType,
    pub connection: &'a ConnectionType,
}

#[allow(deprecated)]
impl<'a> Command<'a> {
    pub fn new(command_type: CommandType, connection: &'a ConnectionType) -> Self {
        Self {
            command_type,
            connection,
        }
    }

    pub fn add(connection: &'a ConnectionType) -> Self {
        Self::new(CommandType::Add, connection)
    }

    pub fn update(connection: &'a ConnectionType) -> Self {
        Self::new(CommandType::Update, connection)
    }

    pub fn delete(connection: &'a ConnectionType) -> Self {
        Self::new(CommandType::Delete, connection)
    }
}

#[deprecated(note = "Use cqrs::prelude::* instead")]
pub trait CommandExecutor
where
    Self: Sized,
{
    type Error;

    #[allow(deprecated)]
    fn perform(self, command: Command) -> Result<(), Self::Error> {
        match command.command_type {
            CommandType::Add => self.add(command),
            CommandType::Update => self.update(command),
            CommandType::Delete => self.delete(command),
        }
    }

    #[allow(deprecated)]
    fn add(self, command: Command) -> Result<(), Self::Error>;
    #[allow(deprecated)]
    fn update(self, command: Command) -> Result<(), Self::Error>;
    #[allow(deprecated)]
    fn delete(self, command: Command) -> Result<(), Self::Error>;
}
