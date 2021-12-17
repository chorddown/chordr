use crate::models::chord::Chord;
use crate::models::metadata::BNotation;

#[derive(Debug, PartialEq, Clone)]
pub enum MetadataIterItemValue {
    Chord(Chord),
    String(String),
    BNotation(BNotation),
    None,
}

impl From<Option<Chord>> for MetadataIterItemValue {
    fn from(v: Option<Chord>) -> Self {
        match v {
            None => MetadataIterItemValue::None,
            Some(c) => MetadataIterItemValue::Chord(c),
        }
    }
}

impl From<Chord> for MetadataIterItemValue {
    fn from(v: Chord) -> Self {
        MetadataIterItemValue::Chord(v)
    }
}

impl From<Option<&Chord>> for MetadataIterItemValue {
    fn from(v: Option<&Chord>) -> Self {
        match v {
            None => MetadataIterItemValue::None,
            Some(c) => MetadataIterItemValue::Chord(c.clone()),
        }
    }
}

impl From<&Chord> for MetadataIterItemValue {
    fn from(v: &Chord) -> Self {
        MetadataIterItemValue::Chord(v.clone())
    }
}

impl From<Option<String>> for MetadataIterItemValue {
    fn from(v: Option<String>) -> Self {
        match v {
            None => MetadataIterItemValue::None,
            Some(c) => MetadataIterItemValue::String(c),
        }
    }
}

impl From<String> for MetadataIterItemValue {
    fn from(v: String) -> Self {
        MetadataIterItemValue::String(v)
    }
}

impl From<Option<&str>> for MetadataIterItemValue {
    fn from(v: Option<&str>) -> Self {
        match v {
            None => MetadataIterItemValue::None,
            Some(c) => MetadataIterItemValue::String(c.to_string()),
        }
    }
}

impl From<&str> for MetadataIterItemValue {
    fn from(v: &str) -> Self {
        MetadataIterItemValue::String(v.to_string())
    }
}

impl From<BNotation> for MetadataIterItemValue {
    fn from(v: BNotation) -> Self {
        MetadataIterItemValue::BNotation(v)
    }
}

impl From<Option<BNotation>> for MetadataIterItemValue {
    fn from(v: Option<BNotation>) -> Self {
        match v {
            None => MetadataIterItemValue::None,
            Some(c) => MetadataIterItemValue::BNotation(c),
        }
    }
}
