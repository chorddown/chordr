use crate::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Default)]
pub struct FileReader {}

impl FileReader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn open_file(&self, source: &Path) -> Result<impl Read, Error> {
        let input = File::open(source)?;
        let buffered = BufReader::new(input);

        Ok(buffered)
    }
}
