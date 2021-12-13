use serde::{Deserialize, Serialize};

use crate::models::chord::Chord;
use crate::models::list::ListEntryTrait;
use crate::models::metadata::*;
use crate::models::song_id::SongIdTrait;

use super::file_type::FileType;
use super::song_data::SongData;
use super::song_id::SongId;

/// Representation of a Song's metadata, used e.g. in the JSON export
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SongMetadata {
    id: SongId,
    #[serde(rename = "type")]
    file_type: FileType,

    #[serde(flatten)]
    metadata: Metadata,
}

impl SongMetadata {
    #[deprecated(note = "Use `new_with_metadata()`")]
    pub fn new(id: SongId, title: String, file_type: FileType) -> Self {
        let mut metadata = Metadata::default();
        metadata.title = Some(title);
        Self {
            id,
            file_type,
            metadata,
        }
    }

    #[deprecated(note = "Use `new_with_metadata()`")]
    pub fn new_with_meta_information<S: Into<String>>(
        id: SongId,
        title: S,
        file_type: FileType,
        metadata: &dyn MetadataTrait,
    ) -> Self {
        let title = title.into();
        Self {
            id,
            file_type,
            metadata: Metadata::from(metadata).with_title(title),
        }
    }

    pub fn new_with_metadata(
        id: SongId,
        file_type: FileType,
        metadata: &dyn MetadataTrait,
    ) -> Self {
        Self {
            id,
            file_type,
            metadata: Metadata::from(metadata),
        }
    }
}

impl MetadataTrait for SongMetadata {
    fn title(&self) -> Option<&str> {
        self.metadata.title()
    }

    fn subtitle(&self) -> Option<&str> {
        self.metadata.subtitle()
    }

    fn artist(&self) -> Option<&str> {
        self.metadata.artist()
    }

    fn composer(&self) -> Option<&str> {
        self.metadata.composer()
    }

    fn lyricist(&self) -> Option<&str> {
        self.metadata.lyricist()
    }

    fn copyright(&self) -> Option<&str> {
        self.metadata.copyright()
    }

    fn album(&self) -> Option<&str> {
        self.metadata.album()
    }

    fn year(&self) -> Option<&str> {
        self.metadata.year()
    }

    fn key(&self) -> Option<&Chord> {
        self.metadata.key()
    }
    fn original_key(&self) -> Option<&Chord> {
        self.metadata.original_key()
    }

    fn time(&self) -> Option<&str> {
        self.metadata.time()
    }

    fn tempo(&self) -> Option<&str> {
        self.metadata.tempo()
    }

    fn duration(&self) -> Option<&str> {
        self.metadata.duration()
    }

    fn capo(&self) -> Option<&str> {
        self.metadata.capo()
    }

    fn original_title(&self) -> Option<&str> {
        self.metadata.original_title()
    }

    fn alternative_title(&self) -> Option<&str> {
        self.metadata.alternative_title()
    }

    fn ccli_song_id(&self) -> Option<&str> {
        self.metadata.ccli_song_id()
    }

    fn b_notation(&self) -> BNotation {
        self.metadata.b_notation()
    }
}

impl SongIdTrait for SongMetadata {}

impl ListEntryTrait for SongMetadata {
    type Id = SongId;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl SongData for SongMetadata {
    fn title(&self) -> String {
        self.metadata
            .title()
            .map(|t| t.to_string())
            .unwrap_or_else(|| self.id().to_string())
    }

    fn file_type(&self) -> FileType {
        self.file_type
    }
}

#[cfg(test)]
mod tests {
    use crate::models::chord::Note;
    use crate::models::file_type::FileType;
    use crate::models::metadata::{BNotation, Metadata};
    use crate::prelude::{SongId, SongMetadata};

    #[test]
    fn serialized_data_layout() {
        let song_metadata = get_test_song_metadata();
        let serialized = serde_json::to_string(&song_metadata).unwrap();
        let expected = get_test_json();
        assert_eq!(serialized, expected)
    }

    #[test]
    fn deserialize() {
        let deserialized = serde_json::from_str::<SongMetadata>(get_test_json()).unwrap();
        let expected = get_test_song_metadata();
        assert_eq!(deserialized, expected)
    }

    fn get_test_song_metadata() -> SongMetadata {
        let metadata = Metadata {
            title: Some("Great song".to_string()),
            subtitle: None,
            artist: Some("Me".to_string()),
            composer: Some("I".to_string()),
            lyricist: Some("Myself".to_string()),
            copyright: Some("Somebody".to_string()),
            album: Some("ME".to_string()),
            year: Some("2021".to_string()),
            key: Some(Note::A.into()),
            original_key: Some(Note::D.into()),
            time: None,
            tempo: Some("90".to_string()),
            duration: None,
            capo: None,
            original_title: None,
            alternative_title: None,
            ccli_song_id: None,
            b_notation: BNotation::H,
        };
        SongMetadata::new_with_metadata(SongId::new("song-test"), FileType::Chorddown, &metadata)
    }

    fn get_test_json() -> &'static str {
        r#"{"id":"song-test","type":"chorddown","title":"Great song","subtitle":null,"artist":"Me","composer":"I","lyricist":"Myself","copyright":"Somebody","album":"ME","year":"2021","key":"A","original_key":"D","time":null,"tempo":"90","duration":null,"capo":null,"original_title":null,"alternative_title":null,"ccli_song_id":null,"b_notation":"H"}"#
    }
}
