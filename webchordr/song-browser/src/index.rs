use std::collections::BTreeMap;

use unicode_segmentation::UnicodeSegmentation;

use libchordr::models::prelude::*;

/// A struct holding the character group to navigate to and the number of matching [Song]s
#[derive(Clone, PartialEq, Debug)]
pub struct Index {
    pub chars: String,
    pub count: usize,
}

impl Index {
    fn new(chars: String, count: usize) -> Self {
        Self { chars, count }
    }
}

/// Return the indexes for the given [Song]s
pub fn build_indexes(songs: Vec<&Song>, root_chars: &str) -> Vec<Index> {
    let prefix_length = root_chars.len() + 1;
    let indexes: Vec<String> = songs
        .iter()
        .map(|s| get_prefix(s.title(), prefix_length))
        .collect();
    //    indexes.sort();

    let mut map: BTreeMap<String, Index> = BTreeMap::new();
    for chars in indexes {
        match map.get_mut(&chars) {
            Some(mut index) => {
                index.count += 1;
            }
            None if chars.is_empty() => { /* do nothing */ }
            None => {
                map.insert(chars.clone(), Index::new(chars.clone(), 1));
            }
        }
    }

    map.values().cloned().collect()
}

fn get_prefix(s: String, index_length: usize) -> String {
    let mut lowercase = s.to_lowercase();
    lowercase.retain(|c| !c.is_whitespace());

    sub_string(&lowercase, index_length)
}

pub(super) fn sub_string(input: &str, length: usize) -> String {
    UnicodeSegmentation::graphemes(input, true)
        .take(length)
        .collect::<String>()
}

pub(super) fn char_count(input: &str) -> usize {
    UnicodeSegmentation::graphemes(input, true)
        .collect::<Vec<&str>>()
        .len()
}
