use std::convert::TryFrom;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use crate::helper::parse_content;
use crate::metadata::metadata_trait::MetadataTrait;
use crate::models::file_type::FileType;
use crate::models::song::Song;
use crate::models::song_id::SongId;
use crate::models::song_metadata::SongMetadata;
use crate::parser::ParserResult;

use super::CatalogBuildError;

impl TryFrom<&Path> for Song {
    type Error = CatalogBuildError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let path_buf = path.to_path_buf();
        if !path.is_file() {
            return Err(CatalogBuildError::new(
                "Given entry is not a file",
                path_buf,
            ));
        }

        let src = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(CatalogBuildError::from_error(e, path_buf)),
        };

        let song_id = SongId::from(path);
        let parser_result = match parse_content(src.as_bytes()) {
            Ok(p) => p,
            Err(e) => return Err(CatalogBuildError::from_error(e, path_buf)),
        };
        let ParserResult { metadata, .. } = parser_result;
        let title = metadata
            .title()
            .unwrap_or_else(|| song_id.as_str())
            .to_owned();
        let file_type = match FileType::try_from(path) {
            Ok(f) => f,
            Err(e) => return Err(CatalogBuildError::from_error(e, path_buf)),
        };
        let metadata =
            SongMetadata::new_with_metadata(song_id, file_type, &metadata.with_title(title));
        Ok(Song::new(metadata, src))
    }
}

impl TryFrom<DirEntry> for Song {
    type Error = CatalogBuildError;

    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        TryFrom::try_from(entry.path().as_path())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::list::ListEntryTrait;
    use crate::models::song_data::SongData;

    use super::*;

    #[test]
    fn test_try_from() {
        let song_path = format!(
            "{}/../webchordr/app/static/songs/swing_low_sweet_chariot.chorddown",
            env!("CARGO_MANIFEST_DIR")
        );
        let song_path = Path::new(&song_path);
        let result = Song::try_from(song_path);
        assert!(result.is_ok(), "{:?}", result.unwrap());
        let song = result.unwrap();
        assert_eq!(SongId::new("swing_low_sweet_chariot.chorddown"), song.id());
        assert_eq!("Swing Low Sweet Chariot", &song.title());
        assert_eq!(FileType::Chorddown, song.file_type());
        assert!(!song.src().is_empty());
    }
}
