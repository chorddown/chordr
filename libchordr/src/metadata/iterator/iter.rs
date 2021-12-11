use crate::metadata::iterator::{MetadataIterItem, FIELDS, FIELDS_LEN};
use crate::models::metadata::MetadataTrait;
use crate::models::song_metadata::SongMetadata;
use crate::parser::Metadata;

/// Iterator for the borrowed `MetaTrait` implementations
pub struct MetadataIterator<'a, T: MetadataTrait> {
    metadata: &'a T,
    fields_cursor: usize,
}

impl<'a, T: MetadataTrait> Iterator for MetadataIterator<'a, T> {
    type Item = MetadataIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fields_cursor < FIELDS_LEN {
            let field = FIELDS[self.fields_cursor];
            self.fields_cursor += 1;

            Some(super::call_field_method(self.metadata, field))
        } else {
            None
        }
    }
}

/// Implement `IntoIterator` for the borrowed `MetaInformation`
impl<'a> IntoIterator for &'a Metadata {
    type Item = MetadataIterItem;
    type IntoIter = MetadataIterator<'a, Metadata>;

    fn into_iter(self) -> Self::IntoIter {
        MetadataIterator {
            metadata: self,
            fields_cursor: 0,
        }
    }
}

/// Implement `IntoIterator` for the borrowed `SongMeta`
impl<'a> IntoIterator for &'a SongMetadata {
    type Item = MetadataIterItem;
    type IntoIter = MetadataIterator<'a, SongMetadata>;

    fn into_iter(self) -> Self::IntoIter {
        MetadataIterator {
            metadata: self,
            fields_cursor: 0,
        }
    }
}
