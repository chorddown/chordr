mod dummy_server_backend;
mod test_value;

use chrono::prelude::*;
pub use dummy_server_backend::DummyServerBackend;
use libchordr::prelude::*;
pub use test_value::TestValue;

pub fn entry<S: Into<String>>(id: S) -> SetlistEntry {
    SetlistEntry::from_song_with_settings(&test_song(id), SongSettings::default())
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

pub(crate) fn get_test_user_password_hidden() -> User {
    User::new(
        Username::new("my-username").unwrap(), // username
        "Daniel".to_string(),                  // first_name
        "Corn".to_string(),                    // last_name
        Password::default(),                   // password
    )
}

pub(crate) fn get_test_setlist(user: User) -> Setlist {
    Setlist::new(
        "My setlist",
        10291,
        user,
        None,
        Some(Utc.ymd(2014, 11, 14).and_hms(8, 9, 10)),
        Utc.ymd(2020, 06, 14).and_hms(16, 26, 20),
        Utc::now(),
        vec![entry("song-1"), entry("song-2"), entry("song-3")],
    )
}
