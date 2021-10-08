pub trait CatalogHandler {
    fn fetch_catalog(&mut self);

    fn commit_changes(&mut self);
}
