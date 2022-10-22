//! Blocking version of the CQRS API
mod command_executor;
mod repository;

pub use command_executor::CommandExecutor;
pub use repository::RepositoryTrait;
