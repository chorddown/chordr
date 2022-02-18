use std::collections::hash_map::Iter;
use std::collections::HashMap;

use crate::format::Format;
use crate::prelude::{
    convert_to_format, Catalog, CatalogTrait, Formatting, ListEntryTrait, Result, Song, SongId,
};

pub struct Index {
    map: HashMap<SongId, String>,
}

impl Index {
    pub(super) fn build_index(catalog: &Catalog) -> Self {
        let mut index = Self::with_capacity(catalog.len());
        for song in catalog.iter() {
            if let Ok(text) = extract_text(song) {
                index.map.insert(song.id(), text.to_lowercase());
            }
        }
        index
    }

    pub(super) fn iter(&self) -> Iter<'_, SongId, String> {
        self.map.iter()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }
}

fn extract_text(s: &Song) -> Result<String> {
    let formatting = Formatting {
        format: Format::Text,
        ..Formatting::default()
    };

    convert_to_format(s.src().as_bytes(), s.meta(), formatting)
}
