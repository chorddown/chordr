mod deserialize;
mod serialize;

use crate::error::Error;
use std::convert::TryFrom;
use std::fmt;
use std::fs::DirEntry;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FileType {
    Chorddown,
    Jpeg,
}

impl FileType {
    pub fn dir_entry_matches(&self, dir_entry: &DirEntry) -> bool {
        self.path_matches(dir_entry.path().as_ref())
    }

    pub fn path_matches(&self, path: &Path) -> bool {
        match path.extension() {
            Some(t) => t == self.str_representation(),
            None => false,
        }
    }

    fn str_representation(&self) -> &str {
        match self {
            FileType::Chorddown => "chorddown",
            FileType::Jpeg => "jpeg",
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str_representation())
    }
}

impl TryFrom<&Path> for FileType {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        if let Some(t) = value.extension() {
            if let Some(e) = t.to_str() {
                return TryFrom::<&str>::try_from(e);
            }
        }

        Err(Error::file_type_error(format!("Could not detect file extension for path {:?}", value)))
    }
}

impl TryFrom<&str> for FileType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Chorddown" | "chorddown" => Ok(FileType::Chorddown),
            "Jpeg" | "jpeg" => Ok(FileType::Jpeg),
            _ => Err(Error::unknown_error(format!("Invalid FileType {}", value)))
        }
    }
}

impl TryFrom<String> for FileType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let serialized = serde_json::to_string(&FileType::Chorddown).unwrap();
        assert_eq!("\"chorddown\"", serialized);

        let serialized = serde_json::to_string(&FileType::Jpeg).unwrap();
        assert_eq!("\"jpeg\"", serialized);
    }


    #[test]
    fn test_deserialize() {
        let deserialized: FileType = serde_json::from_str("\"chorddown\"").unwrap();
        assert_eq!(FileType::Chorddown, deserialized);

        let deserialized: FileType = serde_json::from_str("\"jpeg\"").unwrap();
        assert_eq!(FileType::Jpeg, deserialized);
    }
}
