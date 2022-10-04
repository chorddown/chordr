mod test_value;

use crate::persistence_manager::CommandContext;
use chrono::prelude::*;
use libchordr::prelude::*;
pub use test_value::TestValue;
use webchordr_common::constants::{STORAGE_V2_KEY_SETLIST, TEST_STORAGE_NAMESPACE};

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
        Utc.ymd(2020, 11, 01).and_hms(19, 17, 14),
        vec![entry("song-1"), entry("song-2"), entry("song-3")],
    )
}

pub(super) fn get_test_command_context() -> CommandContext {
    CommandContext::new(TEST_STORAGE_NAMESPACE, STORAGE_V2_KEY_SETLIST)
}
