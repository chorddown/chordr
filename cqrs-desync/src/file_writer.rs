use crate::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Default)]
pub struct FileWriter {}

impl FileWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_generated_file(&self, content: &str, target: &Path) -> Result<(), Error> {
        if target.exists() && !target.is_file() {
            return Err(Error::Path("Given target exists but is not a file", None));
        }
        if let Some(parent) = target.parent() {
            if !parent.is_dir() {
                return Err(Error::Path(
                    "Given target's parent is not a directory",
                    None,
                ));
            }
        }

        let mut target_file = File::create(target)
            .map_err(|e| Error::Path("Could not open target path for writing", Some(e)))?;

        target_file.write_all(content.as_bytes())?;

        Ok(())
    }
}
