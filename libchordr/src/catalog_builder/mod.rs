mod song_from_dir_entry;
mod catalog_build_error;

use crate::error::{Error, Result};
use crate::models::catalog::Catalog;
use crate::models::file_type::FileType;
use crate::models::song::Song;
use crate::models::song_id::SongIdTrait;
use std::convert::TryFrom;
use std::fs::{self, DirEntry};
use std::path::{Path};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
pub use self::catalog_build_error::CatalogBuildError;

/// Catalog Builder provides functions to build a Song Catalog from a given directory
pub struct CatalogBuilder;

pub struct CatalogBuildResult {
    pub catalog: Catalog,
    pub errors: Vec<CatalogBuildError>,
}

impl CatalogBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build_catalog_for_directory<P: AsRef<Path>>(
        &self,
        path: P,
        file_type: FileType,
        recursive: bool,
    ) -> Result<CatalogBuildResult> {
        let path_ref = path.as_ref();
        if !path_ref.is_dir() {
            return Err(Error::catalog_builder_fatal_error(
                "Given path is not a directory",
                path_ref.to_path_buf(),
            ));
        }

        let song_results = self.collect_songs(path_ref, file_type, recursive);
        let (songs, errors) = self.partition_songs(song_results);

        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

        Ok(CatalogBuildResult {
            catalog: Catalog::new(rand_string, songs),
            errors,
        })
    }

    fn partition_songs(&self, song_results: Vec<Result<Song, CatalogBuildError>>) -> (Vec<Song>, Vec<CatalogBuildError>) {
        let (songs, errors): (Vec<_>, Vec<_>) = song_results.into_iter().partition(Result::is_ok);

        let mut songs: Vec<Song> = songs.into_iter().map(Result::unwrap).collect();
        songs.sort_by(|a, b| a.id().cmp(&b.id()));

        (
            songs,
            errors.into_iter().map(Result::unwrap_err).collect::<Vec<CatalogBuildError>>()
        )
    }

    fn collect_songs(
        &self,
        path: &Path,
        file_type: FileType,
        recursive: bool,
    ) -> Vec<Result<Song, CatalogBuildError>> {
        if !path.is_dir() {
            panic!("Given path is not a directory");
        }

        let entry_iterator = match fs::read_dir(path) {
            Ok(i) => i,
            Err(e) => return vec![Err(CatalogBuildError::from_error(e, path))],
        };

        let mut songs = vec![];
        for entry in entry_iterator {
            match entry {
                Ok(entry) => songs.append(&mut self.collect_songs_of_entry(entry, file_type, recursive)),
                Err(error) => songs.push(Err(CatalogBuildError::from_error(error, path)))
            }
        }
        songs
    }

    fn collect_songs_of_entry(
        &self,
        entry: DirEntry,
        file_type: FileType,
        recursive: bool,
    ) -> Vec<Result<Song, CatalogBuildError>> {
        let path = entry.path();
        if path.is_file() {
            if file_type.dir_entry_matches(&entry) {
                vec![Song::try_from(entry)]
            } else {
                // This is **not** an error situation. If `entry` is not of `file_type` skip the entry
                vec![]
            }
        } else if path.is_dir() {
            if recursive {
                self.collect_songs(path.as_path(), file_type, recursive)
            } else {
                vec![]
            }
        } else {
            vec![Err(CatalogBuildError::new("Given path is neither a valid file nor a directory", path))]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::song_data::SongData;
    use crate::models::song_id::SongId;

    #[test]
    fn test_build_catalog_for_directory() {
        let songs_dir = format!("{}/../webchordr/static/songs", env!("CARGO_MANIFEST_DIR"));
        let songs_dir = Path::new(&songs_dir);
        let result =
            CatalogBuilder::new().build_catalog_for_directory(songs_dir, FileType::Chorddown, true);
        assert!(result.is_ok());
        let catalog_and_errors = result.unwrap();
        let catalog = catalog_and_errors.catalog;
        assert!(2 <= catalog.len());

        let song_id = "swing_low_sweet_chariot.chorddown";
        assert!(catalog.contains_id(song_id));
        assert!(catalog.get(song_id).is_some());
        let song = catalog.get(song_id).unwrap();
        assert_eq!("Swing Low Sweet Chariot", song.title());
        assert_eq!(SongId::new(song_id), song.id());
    }

    #[test]
    fn test_build_catalog_for_test_directory() {
        let songs_dir = format!("{}/tests/resources", env!("CARGO_MANIFEST_DIR"));
        let songs_dir = Path::new(&songs_dir);
        let result =
            CatalogBuilder::new().build_catalog_for_directory(songs_dir, FileType::Chorddown, true);
        assert!(result.is_ok());
        let catalog_and_errors = result.unwrap();
        let catalog = catalog_and_errors.catalog;
        assert_eq!(3, catalog.len());

        let song_id = "german-test.chorddown";
        assert!(catalog.contains_id(song_id));
        assert!(catalog.get(song_id).is_some());
        let song = catalog.get(song_id).unwrap();
        assert_eq!("Überschrift", song.title());
        assert_eq!(SongId::new(song_id), song.id());

        let song_id = "song-id-with-spaces.chorddown";
        println!("{:#?}",catalog);
        assert!(catalog.contains_id(song_id));
        assert!(catalog.get(song_id).is_some());
        let song = catalog.get(song_id).unwrap();
        assert_eq!(SongId::new(song_id), song.id());
    }
}
