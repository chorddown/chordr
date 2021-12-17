use crate::metadata::iterator::iter_item_value::MetadataIterItemValue;
use crate::metadata::keyword::MetadataKeyword;
use crate::models::chord::Chord;
use crate::models::metadata::BNotation;

#[derive(Debug, PartialEq, Clone)]
pub struct MetadataIterItem {
    pub keyword: MetadataKeyword,
    pub value: MetadataIterItemValue,
}

macro_rules! impl_from_for_item {
    ($source_type:ty) => {
        impl From<(MetadataKeyword, $source_type)> for MetadataIterItem {
            fn from(v: (MetadataKeyword, $source_type)) -> Self {
                Self {
                    keyword: v.0,
                    value: v.1.into(),
                }
            }
        }
    };
}
macro_rules! impl_from_for_item_and_option {
    ($source_type:ty) => {
        impl_from_for_item!($source_type);
        impl_from_for_item!(Option<$source_type>);
    };
}

impl From<(MetadataKeyword, MetadataIterItemValue)> for MetadataIterItem {
    fn from(v: (MetadataKeyword, MetadataIterItemValue)) -> Self {
        Self {
            keyword: v.0,
            value: v.1,
        }
    }
}

impl_from_for_item_and_option!(Chord);
impl_from_for_item_and_option!(&Chord);
impl_from_for_item_and_option!(String);
impl_from_for_item_and_option!(&str);
impl_from_for_item_and_option!(BNotation);
