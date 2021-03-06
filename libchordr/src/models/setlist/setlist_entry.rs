use crate::models::file_type::FileType;
use crate::models::list::ListEntryTrait;
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

    #[deprecated]
    pub fn from_song<S: SongData>(song: &S) -> Self {
        Self {
            song_id: song.id(),
            file_type: song.file_type(),
            title: song.title(),
            settings: None,
        }
    }

    pub fn from_song_with_settings<S: SongData + ListEntryTrait<Id = SongId>>(
        song: &S,
        settings: SongSettings,
    ) -> Self {
        Self {
            song_id: song.id(),
            file_type: song.file_type(),
            title: song.title(),
            settings: Some(settings),
        }
    }

    pub fn settings(&self) -> Option<SongSettings> {
        self.settings.clone()
    }

    /// Build a clone of the entry with the given [`SongSettings`] assigned
    pub fn with_settings(&self, settings: SongSettings) -> Self {
        Self {
            song_id: self.id(),
            file_type: self.file_type(),
            title: self.title(),
            settings: Some(settings),
        }
    }

    /// Build a clone of the entry without any [`SongSettings`] assigned
    pub fn without_settings(&self) -> Self {
        Self {
            song_id: self.id(),
            file_type: self.file_type(),
            title: self.title(),
            settings: None,
        }
    }
}

impl SongIdTrait for SetlistEntry {}

impl SongData for SetlistEntry {
    fn title(&self) -> String {
        self.title.clone()
    }
    fn file_type(&self) -> FileType {
        self.file_type
    }
}

impl ListEntryTrait for SetlistEntry {
    type Id = SongId;

    fn id(&self) -> Self::Id {
        self.song_id.clone()
    }
}
