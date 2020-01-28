use chrono::{DateTime, FixedOffset};

#[derive(Clone, Debug)]
pub struct FileEntry {
    path: String,
    size: usize,
    modified_date: DateTime<FixedOffset>,
}

#[allow(dead_code)]
impl FileEntry {
    pub fn new<S: Into<String>>(
        path: S,
        size: usize,
        modified_date: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            path: path.into(),
            size,
            modified_date,
        }
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn modified_date(&self) -> DateTime<FixedOffset> {
        self.modified_date
    }
}
