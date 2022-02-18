use std::rc::Rc;

use crate::prelude::{Catalog, CatalogTrait, Song};
use crate::search::index::Index;

mod index;

#[derive(Debug)]
pub struct SearchIndex {
    index: Index,
    catalog: Rc<Catalog>,
}

impl SearchIndex {
    pub fn build_for_catalog(catalog: Rc<Catalog>) -> Self {
        Self {
            index: Index::build_index(&*catalog),
            catalog,
        }
    }

    /// Return the [Song]s from the [Catalog] filtered by [search]
    pub fn search_by_term(&self, search: &str) -> Vec<&Song> {
        if search.is_empty() || search.trim().is_empty() {
            // If the search is empty return all songs
            return self.catalog.iter().collect();
        }

        let search_normalized = search.to_lowercase();

        self.index
            .search(&search_normalized)
            .into_iter()
            .filter_map(|song_id| self.catalog.get(song_id))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::search::SearchIndex;
    use crate::test_helpers::get_test_catalog;

    #[test]
    fn test_search_by_term() {
        let search_index = SearchIndex::build_for_catalog(Rc::new(get_test_catalog()));
        let result = search_index.search_by_term("Toni");

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_search_by_empty_term() {
        let search_index = SearchIndex::build_for_catalog(Rc::new(get_test_catalog()));
        assert_eq!(search_index.search_by_term("").len(), 5);
        assert_eq!(search_index.search_by_term("    ").len(), 5);
        assert_eq!(search_index.search_by_term(" \t \n    ").len(), 5);
    }
}
