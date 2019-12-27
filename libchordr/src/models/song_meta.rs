use super::file_type::FileType;
use super::song_data::SongData;
use super::song_id::SongId;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SongMeta {
    id: SongId,
    title: String,
    #[serde(rename = "type")]
    file_type: FileType,
}

impl SongMeta {
    pub fn new(id: SongId, title: String, file_type: FileType) -> Self {
        Self {
            id,
            title,
            file_type,
        }
    }
}

impl SongData for SongMeta {
    fn id(&self) -> SongId {
        self.id.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn file_type(&self) -> FileType {
        self.file_type
    }
}
