use libchordr::prelude::{
    convert_to_format, Catalog, CatalogTrait, Format, Formatting, Result, Song, SongData,
};

pub struct SearchUtility {}

impl SearchUtility {
    /// Return the [Song]s from the [Catalog] filtered by [self.search]
    pub fn search_catalog_by_term<'a>(catalog: &'a Catalog, search: &str) -> Vec<&'a Song> {
        if search.is_empty() {
            return catalog.iter().collect();
        }

        let search_normalized = search.to_lowercase();

        Self::filter(catalog, |song| {
            if str::contains(&song.title().to_lowercase(), &search_normalized)
                || str::contains(&song.src().to_lowercase(), &search_normalized)
            {
                return true;
            }
            return false;

            // extract_text(song).map_or(false, |text| {
            //     str::contains(&text.to_lowercase(), &search_normalized)
            // })
        })
    }

    pub fn filter<'a, P>(catalog: &Catalog, predicate: P) -> Vec<&Song>
    where
        P: FnMut(&&Song) -> bool,
    {
        catalog.iter().filter(predicate).collect()
    }
}

#[allow(unused)]
fn extract_text(s: &Song) -> Result<String> {
    let mut formatting = Formatting::default();
    formatting.format = Format::Text;

    convert_to_format(s.src(), s.meta(), formatting)
}
