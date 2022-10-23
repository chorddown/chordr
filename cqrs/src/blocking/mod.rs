//! Blocking version of the CQRS API
mod backend;
mod command_executor;
mod query_executor;
mod repository;

pub use backend::BackendTrait;
pub use command_executor::CommandExecutor;
pub use query_executor::QueryExecutor;
pub use repository::RepositoryTrait;
