use crate::prelude::SongData;

pub fn sort_by_title<T: SongData>(collection: &mut [T]) -> &[T] {
    collection.sort_by(|a, b| a.title().partial_cmp(&b.title()).unwrap());
    collection
}

pub trait SongSorting<T: SongData> {
    fn sort_by_title(self) -> Vec<T>;
}
