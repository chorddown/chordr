use std::collections::HashMap;

use crate::format::Format;
use crate::prelude::{
    convert_to_format, Catalog, CatalogTrait, Formatting, ListEntryTrait, Result, Song, SongId,
};

type Storage = HashMap<SongId, String>;

#[derive(Debug)]
pub struct Index {
    map: Storage,
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

    pub(super) fn search(&self, search: &str) -> Vec<&SongId> {
        self.map
            .iter()
            .filter_map(|(song_id, text)| {
                if text.contains(search) {
                    Some(song_id)
                } else {
                    None
                }
            })
            .collect()
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
