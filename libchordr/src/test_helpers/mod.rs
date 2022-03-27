pub use parsing::*;

use crate::models::user::{User, Username};
use crate::prelude::{Catalog, FileType, Password, Song, SongMeta};

mod parsing;

pub fn get_test_user() -> User {
    User::new(
        Username::new("my-username").unwrap(), // username
        "Daniel".to_string(),                  // first_name
        "Corn".to_string(),                    // last_name
        Password::new("mypass123").unwrap(),   // password
    )
}

pub fn get_test_catalog() -> Catalog {
    let songs = vec![
        Song::new(
            SongMeta::new("song-1".into(), "Song 1".into(), FileType::Chorddown),
            include_str!("../../tests/resources/catalog/song-1.chorddown"),
        ),
        Song::new(
            SongMeta::new("song-2".into(), "Song 2".into(), FileType::Chorddown),
            include_str!("../../tests/resources/catalog/song-2.chorddown"),
        ),
        Song::new(
            SongMeta::new("song-3".into(), "Song 3".into(), FileType::Chorddown),
            include_str!("../../tests/resources/catalog/song-3.chorddown"),
        ),
        Song::new(
            SongMeta::new("song-40".into(), "Song 40".into(), FileType::Chorddown),
            include_str!("../../tests/resources/swing_low_sweet_chariot.chorddown"),
        ),
        Song::new(
            SongMeta::new("song-50".into(), "Song 50".into(), FileType::Chorddown),
            include_str!("../../tests/resources/german-test.chorddown"),
        ),
    ];
    Catalog::new("test-catalog", songs)
}
