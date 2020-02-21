use crate::task::{TaskTrait, RecurringTaskTrait};
use crate::error::Error;
use crate::configuration::Configuration;
use libchordr::prelude::{CatalogBuilder, FileType};
use std::fs;
use log::info;

pub struct BuildCatalogTask {
    catalog_builder: CatalogBuilder,
    configuration: Configuration,
}

impl TaskTrait for BuildCatalogTask {
    fn with_configuration(configuration: &Configuration) -> Result<Self, Error> where Self: std::marker::Sized {
        let catalog_builder = CatalogBuilder::new();
        Ok(Self {
            catalog_builder,
            configuration: configuration.clone(),
        })
    }
}

impl RecurringTaskTrait for BuildCatalogTask {
    fn run(&self) -> Result<(), Error> {
        info!("Run Build Catalog Task");
        let pretty = true;
        let catalog = self.catalog_builder.build_catalog_for_directory(
            self.configuration.output_directory.as_path(),
            FileType::Chorddown,
            true,
        )?;

        let serialization_result = if pretty {
            serde_json::to_string_pretty(&catalog)
        } else {
            serde_json::to_string(&catalog)
        };

        let output = match serialization_result {
            Ok(s) => s,
            Err(e) => return Err(Error::serialization_error(format!("{}", e))),
        };

        info!("Write catalog to {}", self.configuration.catalog_file.as_path().to_string_lossy());
        Ok(fs::write(self.configuration.catalog_file.as_path(), output)?)
    }
}
