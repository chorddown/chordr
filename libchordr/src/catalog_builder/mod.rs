use std::convert::TryFrom;
use std::fmt::Debug;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;

use chrono::Local;

use crate::error::{Error, Result};
use crate::models::catalog::*;
use crate::models::file_type::FileType;
use crate::models::list::ListEntryTrait;
use crate::models::song::Song;

pub use self::catalog_build_error::CatalogBuildError;

mod catalog_build_error;
mod song_from_dir_entry;

pub struct CatalogBuildResult {
    pub catalog: Catalog,
    pub errors: Vec<CatalogBuildError>,
}

/// Catalog Builder provides functions to build a Song Catalog from a given directory
#[derive(Default)]
pub struct CatalogBuilder;

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

        let song_files_r: Vec<Result<PathBuf, CatalogBuildError>> =
            self.collect_song_files(path_ref, file_type, recursive);
        let (song_file_results, io_errors): (Vec<_>, Vec<_>) = partition_results(song_files_r);
        let song_results = self.build_songs_for_file_list(song_file_results);
        let (songs, mut parse_errors) = self.partition_songs(song_results);

        parse_errors.extend(io_errors.into_iter());

        Ok(CatalogBuildResult {
            catalog: Catalog::new(Local::now().to_rfc2822(), songs),
            errors: parse_errors,
        })
    }

    #[cfg(not(feature = "parallel_catalog_builder"))]
    fn build_songs_for_file_list(
        &self,
        song_file_results: Vec<PathBuf>,
    ) -> Vec<Result<Song, CatalogBuildError>> {
        song_file_results
            .into_iter()
            .map(|e| Song::try_from(e.as_path()))
            .collect()
    }

    #[cfg(feature = "parallel_catalog_builder")]
    fn build_songs_for_file_list(
        &self,
        song_file_results: Vec<PathBuf>,
    ) -> Vec<Result<Song, CatalogBuildError>> {
        use rayon::prelude::*;
        song_file_results
            .into_par_iter()
            .map(|e| Song::try_from(e.as_path()))
            .collect()
    }

    fn partition_songs(
        &self,
        song_results: Vec<Result<Song, CatalogBuildError>>,
    ) -> (Vec<Song>, Vec<CatalogBuildError>) {
        let (mut songs, errors) = partition_results(song_results);

        songs.sort_by_key(|a| a.id());

        (songs, errors)
    }

    fn collect_song_files(
        &self,
        path: &Path,
        file_type: FileType,
        recursive: bool,
    ) -> Vec<Result<PathBuf, CatalogBuildError>> {
        if !path.is_dir() {
            panic!("Given path is not a directory");
        }

        let entry_iterator = match fs::read_dir(path) {
            Ok(i) => i,
            Err(e) => return vec![Err(CatalogBuildError::from_error(Error::io_error(e), path))],
        };

        let mut songs = vec![];
        for entry in entry_iterator {
            match entry {
                Ok(entry) => {
                    songs.append(&mut self.collect_songs_of_entry(entry, file_type, recursive))
                }
                Err(error) => songs.push(Err(CatalogBuildError::from_error(error, path))),
            }
        }
        songs
    }

    fn collect_songs_of_entry(
        &self,
        entry: DirEntry,
        file_type: FileType,
        recursive: bool,
    ) -> Vec<Result<PathBuf, CatalogBuildError>> {
        let path = entry.path();
        if path.is_file() {
            if file_type.dir_entry_matches(&entry) {
                vec![Ok(entry.path())]
            } else {
                // This is **not** an error situation. If `entry` is not of `file_type` skip the entry
                vec![]
            }
        } else if path.is_dir() {
            if recursive {
                self.collect_song_files(path.as_path(), file_type, recursive)
            } else {
                vec![]
            }
        } else {
            vec![Err(CatalogBuildError::new(
                "Given path is neither a valid file nor a directory",
                path,
            ))]
        }
    }
}

pub fn partition_results<T: Debug, E: Debug>(results: Vec<Result<T, E>>) -> (Vec<T>, Vec<E>) {
    let mut left: Vec<T> = Vec::with_capacity(results.len() / 2);
    let mut right: Vec<E> = Vec::with_capacity(results.len() / 2);

    results.into_iter().fold((), |(), x| match x {
        Ok(o) => left.push(o),
        Err(e) => right.push(e),
    });

    (left, right)
}

#[cfg(test)]
mod tests {
    use crate::models::song_data::SongData;
    use crate::models::song_id::SongId;

    use super::*;

    #[test]
    fn test_build_catalog_for_directory() {
        let songs_dir = format!(
            "{}/../webchordr/app/static/songs",
            env!("CARGO_MANIFEST_DIR")
        );
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
        let result = CatalogBuilder::new().build_catalog_for_directory(
            songs_dir,
            FileType::Chorddown,
            false,
        );
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
        assert!(catalog.contains_id(song_id));
        assert!(catalog.get(song_id).is_some());
        let song = catalog.get(song_id).unwrap();
        assert_eq!(SongId::new(song_id), song.id());
    }

    #[test]
    fn test_build_catalog_for_test_directory_recursive() {
        let songs_dir = format!("{}/tests/resources", env!("CARGO_MANIFEST_DIR"));
        let songs_dir = Path::new(&songs_dir);
        let result =
            CatalogBuilder::new().build_catalog_for_directory(songs_dir, FileType::Chorddown, true);
        assert!(result.is_ok());
        let catalog_and_errors = result.unwrap();
        let catalog = catalog_and_errors.catalog;
        assert_eq!(6, catalog.len());

        let song_id = "song-1.chorddown";
        assert!(catalog.contains_id(song_id));
        assert!(catalog.get(song_id).is_some());
        let song = catalog.get(song_id).unwrap();
        assert_eq!("Song 1", song.title());
        assert_eq!(SongId::new(song_id), song.id());

        let song_id = "song-id-with-spaces.chorddown";
        assert!(catalog.contains_id(song_id));
        assert!(catalog.get(song_id).is_some());
        let song = catalog.get(song_id).unwrap();
        assert_eq!(SongId::new(song_id), song.id());
    }
}
