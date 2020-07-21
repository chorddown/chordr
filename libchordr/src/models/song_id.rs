use crate::models::list::ListEntryTrait;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::Path;

/// Trait for objects that have an associated SongId
pub trait SongIdTrait: ListEntryTrait<Id = SongId> {}

/// Song Identifier
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SongId(String);

impl SongId {
    /// Build a new identifier from the given input
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            0: input.into().replace(" ", "-"),
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Eq for SongId {}

impl PartialEq for SongId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for SongId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for SongId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for SongId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl Display for SongId {
    fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&Path> for SongId {
    fn from(path: &Path) -> Self {
        SongId::new(path.file_name().unwrap().to_str().unwrap().to_owned())
    }
}

impl ::std::convert::From<&str> for SongId {
    fn from(input: &str) -> Self {
        SongId::new(input.to_owned())
    }
}

impl ::std::convert::AsRef<str> for SongId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl ::std::str::FromStr for SongId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SongId::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let ids = vec![
            "identifier-test.chorddown",
            "something_else.chorddown",
            "abcdefg",
        ];
        for id in ids {
            assert_eq!(
                serde_json::to_string(&SongId::new(id)).unwrap(),
                format!("\"{}\"", id)
            );
        }
    }

    #[test]
    fn test_deserialize() {
        let ids = vec![
            "identifier-test.chorddown",
            "something_else.chorddown",
            "abcdefg",
        ];
        for id in ids {
            assert_eq!(
                serde_json::from_str::<SongId>(&format!("\"{}\"", id)).unwrap(),
                SongId::new(id)
            );
        }
    }

    #[test]
    fn test_new() {
        let ids = vec![
            ("identifier test.chorddown", "identifier-test.chorddown"),
            ("identifier-test.chorddown", "identifier-test.chorddown"),
            ("something_else.chorddown", "something_else.chorddown"),
            ("Übungslied.chorddown", "Übungslied.chorddown"),
            ("abcdefg", "abcdefg"),
        ];
        for (input, id) in ids {
            assert_eq!(id, SongId::new(input).as_str());
        }
    }
}
