use libchordr::models::file_type::FileType;
use libchordr::models::song_id::SongId;
use libchordr::models::user::{Password, User, Username};
use libchordr::prelude::{ListEntryTrait, SetlistEntry, SongData, SongIdTrait};

pub fn entry<S: Into<String>>(id: S) -> SetlistEntry {
    SetlistEntry::from_song(&test_song(id))
}

pub fn test_song<S: Into<String>>(id: S) -> TestSong {
    TestSong { id: id.into() }
}

pub struct TestSong {
    id: String,
}

impl SongIdTrait for TestSong {}

impl ListEntryTrait for TestSong {
    type Id = SongId;
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

pub(crate) fn get_test_user() -> User {
    User::new(
        Username::new("my-username").unwrap(), // username
        "Daniel".to_string(),                  // first_name
        "Corn".to_string(),                    // last_name
        Password::new("mypass123").unwrap(),   // password
    )
}
