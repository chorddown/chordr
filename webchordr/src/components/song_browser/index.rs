use libchordr::models::prelude::*;
use std::collections::BTreeMap;
use log::debug;

/// A struct holding the character group to navigate to and the number of matching [Song]s
#[derive(Clone, PartialEq, Debug)]
pub struct Index {
    pub chars: String,
    pub count: usize,
}

impl Index {
    fn new(chars: String, count: usize) -> Self {
        Self {
            chars,
            count,
        }
    }
}

/// Return the indexes for the given [Song]s
pub fn build_indexes(songs: Vec<&Song>, root_chars: &str) -> Vec<Index> {
    let indexes: Vec<String> = songs.iter().map(|s| get_index(s.title(), root_chars)).collect();
//    indexes.sort();

    let mut map: BTreeMap<String, Index> = BTreeMap::new();
    for chars in indexes {
        match map.get_mut(&chars) {
            Some(mut index) => { index.count += 1; }
            None => { map.insert(chars.clone(), Index::new(chars.clone(), 1)); }
        }
    }

    map.values().cloned().collect()
}

/// Get the index of [s] according to the current [props.chars]
fn get_index(s: String, root_chars: &str) -> String {
    debug!("root_chars: {}", root_chars);
    let len = if root_chars.is_empty() {
        1
    } else {
        root_chars.len() + 1
    };
    let mut lowercase = s.to_lowercase();
    lowercase.retain(|c| !c.is_whitespace());

    sub_string(&lowercase, len)
}

fn sub_string(input: &str, length: usize) -> String {
    input.chars().take(length).collect()
}

