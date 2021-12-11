use crate::models::chord::Chord;
use crate::models::metadata::BNotation;

#[derive(Debug, PartialEq, Clone)]
pub enum MetadataIterItem {
    Chord(Chord),
    String(String),
    BNotation(BNotation),
    None,
}

impl From<Option<Chord>> for MetadataIterItem {
    fn from(v: Option<Chord>) -> Self {
        match v {
            None => MetadataIterItem::None,
            Some(c) => MetadataIterItem::Chord(c),
        }
    }
}

impl From<Chord> for MetadataIterItem {
    fn from(v: Chord) -> Self {
        MetadataIterItem::Chord(v)
    }
}

impl From<Option<&Chord>> for MetadataIterItem {
    fn from(v: Option<&Chord>) -> Self {
        match v {
            None => MetadataIterItem::None,
            Some(c) => MetadataIterItem::Chord(c.clone()),
        }
    }
}

impl From<&Chord> for MetadataIterItem {
    fn from(v: &Chord) -> Self {
        MetadataIterItem::Chord(v.clone())
    }
}

impl From<Option<String>> for MetadataIterItem {
    fn from(v: Option<String>) -> Self {
        match v {
            None => MetadataIterItem::None,
            Some(c) => MetadataIterItem::String(c),
        }
    }
}

impl From<String> for MetadataIterItem {
    fn from(v: String) -> Self {
        MetadataIterItem::String(v)
    }
}

impl From<Option<&str>> for MetadataIterItem {
    fn from(v: Option<&str>) -> Self {
        match v {
            None => MetadataIterItem::None,
            Some(c) => MetadataIterItem::String(c.to_string()),
        }
    }
}

impl From<&str> for MetadataIterItem {
    fn from(v: &str) -> Self {
        MetadataIterItem::String(v.to_string())
    }
}

impl From<BNotation> for MetadataIterItem {
    fn from(v: BNotation) -> Self {
        MetadataIterItem::BNotation(v)
    }
}
