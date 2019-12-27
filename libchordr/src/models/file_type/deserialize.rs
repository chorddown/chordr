use serde::{Deserialize, Deserializer};

use std::fmt;

use super::FileType;
use serde::de::{self, Visitor};
use std::convert::TryFrom;

struct FileTypeVisitor;

impl<'de> Visitor<'de> for FileTypeVisitor {
    type Value = FileType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one of the strings \"chorddown\" or \"jpeg\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match FileType::try_from(v) {
            Ok(t) => Ok(t),
            Err(e) => Err(E::custom(format!("{}", e))),
        }
    }
}

impl<'de> Deserialize<'de> for FileType {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FileTypeVisitor)
    }
}
