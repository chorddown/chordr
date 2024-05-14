use crate::format::Format;
use crate::models::meta::Tags;
use crate::prelude::{
    convert_to_format, Catalog, CatalogTrait, Formatting, ListEntryTrait, MetaTrait, Result, Song,
    SongId,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct IndexEntry {
    tags: Tags,
    text: String,
}

impl IndexEntry {
    fn search_in_tags(&self, search: &str) -> bool {
        if self.tags.is_empty() {
            return false;
        }

        // Try to parse the whole search string into a list of Tags
        let search_term_as_tags = match Tags::from_str(search) {
            Ok(t) => t,
            Err(_) => return false,
        };

        for search_tag in search_term_as_tags.iter() {
            if self
                .tags
                .iter()
                .find(|t| t.as_str().to_lowercase() == search_tag.as_str())
                .is_none()
            {
                // Return false if one of the searched tags was NOT found
                return false;
            }
        }

        true
    }

    fn contains(&self, search: &str) -> bool {
        if search.starts_with('#') {
            if self.search_in_tags(search) {
                return true;
            }
        }

        self.text.contains(search)
    }
}

type Storage = HashMap<SongId, IndexEntry>;

#[derive(Debug)]
pub struct Index {
    map: Storage,
}

impl Index {
    pub(super) fn build_index(catalog: &Catalog) -> Self {
        let mut index = Self::with_capacity(catalog.len());
        for song in catalog.iter() {
            if let Ok(text) = extract_text(song) {
                index.map.insert(
                    song.id(),
                    IndexEntry {
                        text: text.to_lowercase(),
                        tags: song.meta().tags(),
                    },
                );
            }
        }
        index
    }

    pub(super) fn search(&self, search: &str) -> Vec<&SongId> {
        self.map
            .iter()
            .filter_map(|(song_id, entry)| {
                if entry.contains(search) {
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
