mod collection_task;
mod download_task;
mod build_catalog_task;

use crate::configuration::Configuration;
use crate::error::Result;
pub use self::download_task::DownloadTask;
pub use self::build_catalog_task::BuildCatalogTask;
pub use self::collection_task::CollectionTask;

pub trait TaskTrait {
    fn with_configuration(configuration: &Configuration) -> Result<Self>
        where Self: std::marker::Sized;
}

pub trait RecurringTaskTrait: TaskTrait {
    fn run(&self) -> Result<()>;
}

