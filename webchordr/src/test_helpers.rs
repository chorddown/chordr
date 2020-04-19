use libchordr::prelude::{SongData, SongIdTrait, SetlistEntry};
use libchordr::models::file_type::FileType;
use libchordr::models::song_id::SongId;

pub fn entry<S: Into<String>>(id: S) -> SetlistEntry {
    SetlistEntry::from_song(&TestSong { id: id.into() })
}

struct TestSong {
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

