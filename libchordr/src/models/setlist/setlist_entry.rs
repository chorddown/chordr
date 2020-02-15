use crate::models::song::Song;
use crate::models::song_data::SongData;
use crate::models::file_type::FileType;
use crate::models::song_id::{SongId, SongIdTrait};
use serde::{Serialize, Deserialize};

/// An implementation of [SongData] for use inside [Setlist]s
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SetlistEntry {
    song_id: SongId,
    file_type: FileType,
    title: String,
}

impl SetlistEntry {
    pub fn from_song<S: SongData>(song: &S) -> Self {
        Self {
            song_id: song.id(),
            file_type: song.file_type(),
            title: song.title(),
        }
    }
}

impl From<Song> for SetlistEntry {
    fn from(s: Song) -> Self {
        SetlistEntry::from_song(&s)
    }
}

impl From<&Song> for SetlistEntry {
    fn from(s: &Song) -> Self {
        SetlistEntry::from_song(s)
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
