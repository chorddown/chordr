mod build_catalog_task;
mod collection_task;
mod download_task;

pub use self::build_catalog_task::BuildCatalogTask;
pub use self::collection_task::CollectionTask;
pub use self::download_task::DownloadTask;
use crate::configuration::Configuration;
use crate::error::Result;

pub trait TaskTrait {
    fn with_configuration(configuration: Configuration) -> Result<Self>
    where
        Self: Sized;
}

pub trait RecurringTaskTrait: TaskTrait {
    fn run(&self) -> Result<()>;
}
