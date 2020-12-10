use crate::models::setlist::Setlist;
use serde::{Deserialize, Serialize};
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetlistCollection(Vec<Setlist>);

impl SetlistCollection {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, Setlist> {
        self.0.iter()
    }
}

impl Default for SetlistCollection {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for SetlistCollection {
    type Item = Setlist;
    type IntoIter = IntoIter<Setlist>;

    #[inline]
    fn into_iter(self) -> IntoIter<Setlist> {
        self.0.into_iter()
    }
}
