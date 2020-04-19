use libchordr::models::file_type::FileType;
use libchordr::models::song_id::SongId;
use libchordr::prelude::{SetlistEntry, SongData, SongIdTrait};

pub fn entry<S: Into<String>>(id: S) -> SetlistEntry {
    SetlistEntry::from_song(&test_song(id))
}

pub fn test_song<S: Into<String>>(id: S) -> TestSong {
    TestSong { id: id.into() }
}

pub struct TestSong {
    id: String,
}

impl SongIdTrait for TestSong {
    fn id(&self) -> SongId {
        self.id.as_str().into()
    }
}

impl SongData for TestSong {
    fn title(&self) -> String {
        self.id.clone()
    }

    fn file_type(&self) -> FileType {
        FileType::Chorddown
    }
}
