use crate::models::song::Song;
use crate::models::song_data::SongData;
use crate::models::file_type::FileType;
use crate::models::chord::fmt::Formatting;
use crate::models::song_id::{SongId, SongIdTrait};
use serde::Deserialize;
use serde::Serialize;
use crate::models::song_settings::SongSettings;

/// An implementation of [SongData] for use inside [Setlist]s
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SetlistEntry {
    song_id: SongId,
    file_type: FileType,
    title: String,
    settings: SongSettings,
}

impl SetlistEntry {
    pub fn from_song_with_settings<S: SongData>(song: &S, settings: SongSettings) -> Self {
        Self {
            song_id: song.id(),
            file_type: song.file_type(),
            title: song.title(),
            settings,
        }
    }

    pub fn from_song<S: SongData>(song: &S, formatting: Formatting) -> Self {
        Self::from_song_with_settings(song, SongSettings::new(0, formatting))
    }

    pub fn from_song_with_formatting_and_transpose<S: SongData>(song: &S, formatting: Formatting, transpose: isize) -> Self {
        Self::from_song_with_settings(song, SongSettings::new(transpose, formatting))
    }
}

impl From<Song> for SetlistEntry {
    fn from(s: Song) -> Self {
        SetlistEntry::from_song(&s, Formatting::default())
    }
}

impl SongIdTrait for SetlistEntry {
    fn id(&self) -> String {
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
