mod song_from_dir_entry;

use crate::error::{Error, Result};
use crate::models::catalog::Catalog;
use crate::models::file_type::FileType;
use crate::models::song::Song;
use std::convert::TryFrom;
use std::fs::{self, DirEntry};
use std::path::Path;
use crate::models::song_data::SongData;

/// Catalog Builder provides functions to build a Song Catalog from a given directory
pub struct CatalogBuilder;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


impl CatalogBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build_catalog_for_directory<P: AsRef<Path>>(
        &self,
        path: P,
        file_type: FileType,
        recursive: bool,
    ) -> Result<Catalog> {
        let mut songs: Vec<Song> = self.collect_songs(path.as_ref(), file_type, recursive)?;

        songs.sort_by(|a, b| a.id().cmp(&b.id()));

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();
        Ok(Catalog::new(rand_string, songs))
    }

    fn collect_songs(
        &self,
        path: &Path,
        file_type: FileType,
        recursive: bool,
    ) -> Result<Vec<Song>> {
        if !path.is_dir() {
            return Err(Error::catalog_builder_error(
                "Given path is not a directory",
                path.to_path_buf(),
            ));
        }

        let mut songs = vec![];
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            songs.append(&mut self.collect_songs_of_entry(entry, file_type, recursive)?);
        }
        Ok(songs)
    }

    fn collect_songs_of_entry(
        &self,
        entry: DirEntry,
        file_type: FileType,
        recursive: bool,
    ) -> Result<Vec<Song>> {
        let path = entry.path();
        if path.is_file() {
            if file_type.dir_entry_matches(&entry) {
                let song = Song::try_from(entry)?;

                Ok(vec![song])
            } else {
                // This is **not** an error situation. If `entry` does not is of `file_type` skip the entry
                Ok(vec![])
            }
        } else if path.is_dir() {
            if recursive {
                self.collect_songs(path.as_path(), file_type, recursive)
            } else {
                Ok(vec![])
            }
        } else {
            Err(Error::catalog_builder_error(
                "Given path is not a directory",
                path,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::song_data::SongData;


    #[test]
    fn test_build_catalog_for_directory() {
        let songs_dir = format!("{}/../webchordr/static/songs", env!("CARGO_MANIFEST_DIR"));
        let songs_dir = Path::new(&songs_dir);
        let result = CatalogBuilder::new().build_catalog_for_directory(
            songs_dir,
            FileType::Chorddown,
            true,
        );
        assert!(result.is_ok());
        let catalog = result.unwrap();
        assert!(2 <= catalog.len());

        let song_id = "swing_low_sweet_chariot.chorddown";
        assert!(catalog.contains_id(song_id));
        assert!(catalog.get(song_id).is_some());
        let song = catalog.get(song_id).unwrap();
        assert_eq!("Swing Low Sweet Chariot", song.title());
        assert_eq!(song_id, song.id());
    }
}
