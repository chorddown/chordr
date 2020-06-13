use crate::models::file_type::FileType;
use crate::models::song_data::SongData;
use crate::models::song_id::{SongId, SongIdTrait};
use crate::models::song_settings::SongSettings;
use serde::{Deserialize, Serialize};

/// An implementation of [SongData] for use inside [Setlist]s
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SetlistEntry {
    song_id: SongId,
    file_type: FileType,
    title: String,
    settings: Option<SongSettings>,
}

impl SetlistEntry {
    pub fn new<I: Into<SongId>, S: Into<String>>(
        song_id: I,
        file_type: FileType,
        title: S,
        settings: Option<SongSettings>,
    ) -> Self {
        Self {
            song_id: song_id.into(),
            file_type,
            title: title.into(),
            settings,
        }
    }

    pub fn from_song<S: SongData>(song: &S) -> Self {
        Self {
            song_id: song.id(),
            file_type: song.file_type(),
            title: song.title(),
            settings: None,
        }
    }

    pub fn from_song_with_settings<S: SongData>(song: &S, settings: SongSettings) -> Self {
        let mut entry = Self::from_song(song);
        entry.settings = Some(settings);
        entry
    }
}

// impl From<Song> for SetlistEntry {
//     fn from(s: Song) -> Self {
//         SetlistEntry::from_song(&s)
//     }
// }

// impl<S: SongData> From<&S> for SetlistEntry {
//     fn from(s: &S) -> Self {
//         SetlistEntry::from_song(s)
//     }
// }

impl SongIdTrait for SetlistEntry {
    fn id(&self) -> SongId {
        self.song_id.clone()
    }
}

impl SongData for SetlistEntry {
    fn title(&self) -> String {
        self.title.clone()
    }
    fn file_type(&self) -> FileType {
        self.file_type
    }
}
