use crate::models::meta::BNotation;
use crate::models::user::{User, Username};
use crate::parser::{MetaInformation, Node};
use crate::prelude::{Catalog, FileType, Password, Song, SongMeta};
use crate::tokenizer::{Modifier, Token};

#[cfg(test)]
pub fn get_test_parser_input() -> Vec<Token> {
    vec![
        Token::headline(1, "Swing Low Sweet Chariot", Modifier::None),
        Token::newline(),
        Token::headline(2, "Chorus", Modifier::Chorus),
        Token::literal("Swing "),
        Token::chord("D"),
        Token::literal("low, sweet "),
        Token::chord("G"),
        Token::literal("chari"),
        Token::chord("D"),
        Token::literal("ot,"),
        Token::literal("Comin’ for to carry me "),
        Token::chord("A7"),
        Token::literal("home."),
        Token::literal("Swing "),
        Token::chord("D7"),
        Token::headline(2, "Verse", Modifier::None),
        Token::chord("E"),
        Token::literal("low, sweet "),
        Token::chord("G"),
        Token::literal("chari"),
        Token::chord("D"),
        Token::literal("ot,"),
        Token::chord("E"),
        Token::chord("A"),
        Token::newline(),
        Token::chord("B"),
        Token::chord("H"),
    ]
}

pub fn get_test_tokens() -> Vec<Token> {
    vec![
        Token::headline(1, "Swing Low Sweet Chariot", Modifier::None),
        Token::newline(),
        Token::newline(),
        Token::headline(2, "Chorus", Modifier::Chorus),
        Token::newline(),
        Token::literal("Swing "),
        Token::chord("D"),
        Token::literal("low, sweet "),
        Token::chord("G"),
        Token::literal("chari"),
        Token::chord("D"),
        Token::literal("ot,"),
        Token::newline(),
        Token::literal("Comin’ for to carry me "),
        Token::chord("A7"),
        Token::literal("home."),
        Token::newline(),
        Token::literal("Swing "),
        Token::chord("D7"),
        Token::literal("low, sweet "),
        Token::chord("G"),
        Token::literal("chari"),
        Token::chord("D"),
        Token::literal("ot,"),
        Token::newline(),
        Token::literal("Comin’ for to "),
        Token::chord("A7"),
        Token::literal("carry me "),
        Token::chord("D"),
        Token::literal("home."),
        Token::newline(),
        Token::newline(),
        Token::newline(),
        Token::headline(2, "Verse 1", Modifier::None),
        Token::newline(),
        Token::newline(),
        Token::literal("I "),
        Token::chord("D"),
        Token::literal("looked over Jordan, and "),
        Token::chord("G"),
        Token::literal("what did I "),
        Token::chord("D"),
        Token::literal("see,"),
        Token::newline(),
        Token::literal("Comin’ for to carry me "),
        Token::chord("A7"),
        Token::literal("home."),
        Token::newline(),
        Token::literal("A "),
        Token::chord("D"),
        Token::literal("band of angels "),
        Token::chord("G"),
        Token::literal("comin’ after "),
        Token::chord("D"),
        Token::literal("me,"),
        Token::newline(),
        Token::literal("Comin’ for to "),
        Token::chord("A7"),
        Token::literal("carry me "),
        Token::chord("D"),
        Token::literal("home."),
        Token::newline(),
        Token::newline(),
        Token::quote("Chorus"),
        Token::newline(),
    ]
}

pub fn get_test_ast() -> Node {
    Node::Document(vec![
        Node::section(
            1,
            "Swing Low Sweet Chariot",
            Modifier::None,
            vec![Node::newline()],
        ),
        Node::section(
            2,
            "Chorus",
            Modifier::Chorus,
            vec![
                Node::newline(),
                Node::text("Swing "),
                Node::chord_text_pair("D", "low, sweet ").unwrap(),
                Node::chord_text_pair("G", "chari").unwrap(),
                Node::chord_text_pair_last_in_line("D", "ot,").unwrap(),
                Node::newline(),
                Node::text("Comin’ for to carry me "),
                Node::chord_text_pair_last_in_line("A7", "home.").unwrap(),
                Node::newline(),
                Node::text("Swing "),
                Node::chord_text_pair("D7", "low, sweet ").unwrap(),
                Node::chord_text_pair("G", "chari").unwrap(),
                Node::chord_text_pair_last_in_line("D", "ot,").unwrap(),
                Node::newline(),
                Node::text("Comin’ for to "),
                Node::chord_text_pair("A7", "carry me ").unwrap(),
                Node::chord_text_pair_last_in_line("D", "home.").unwrap(),
                Node::newline(),
            ],
        ),
        Node::section(
            2,
            "Verse 1",
            Modifier::None,
            vec![
                Node::newline(),
                Node::text("I "),
                Node::chord_text_pair("D", "looked over Jordan, and ").unwrap(),
                Node::chord_text_pair("G", "what did I ").unwrap(),
                Node::chord_text_pair_last_in_line("D", "see,").unwrap(),
                Node::newline(),
                Node::text("Comin’ for to carry me "),
                Node::chord_text_pair_last_in_line("A7", "home.").unwrap(),
                Node::newline(),
                Node::text("A "),
                Node::chord_text_pair("D", "band of angels ").unwrap(),
                Node::chord_text_pair("G", "comin’ after ").unwrap(),
                Node::chord_text_pair_last_in_line("D", "me,").unwrap(),
                Node::newline(),
                Node::text("Comin’ for to "),
                Node::chord_text_pair("A7", "carry me ").unwrap(),
                Node::chord_text_pair_last_in_line("D", "home.").unwrap(),
                Node::newline(),
            ],
        ),
        Node::quote("Chorus"),
        Node::newline(),
    ])
}

pub fn get_test_ast_with_quote() -> Node {
    Node::Document(vec![
        Node::section(
            1,
            "Swing Low Sweet Chariot",
            Modifier::None,
            vec![Node::newline()],
        ),
        Node::quote("Play slowly"),
        Node::newline(),
        Node::section(
            2,
            "Chorus",
            Modifier::Chorus,
            vec![
                Node::newline(),
                Node::text("Swing "),
                Node::chord_text_pair("D", "low, sweet ").unwrap(),
                Node::chord_text_pair("G", "chari").unwrap(),
                Node::chord_text_pair("D", "ot.").unwrap(),
            ],
        ),
    ])
}

pub fn get_test_ast_w_inline_metadata() -> Node {
    Node::Document(vec![
        Node::section(
            1,
            "Swing Low Sweet Chariot",
            Modifier::None,
            vec![Node::newline()],
        ),
        Node::meta("Artist: The Fantastic Corns").unwrap(),
        Node::newline(),
        Node::meta("Composer: Daniel Corn").unwrap(),
        Node::newline(),
        Node::section(
            2,
            "Chorus",
            Modifier::Chorus,
            vec![
                Node::newline(),
                Node::text("Swing "),
                Node::chord_text_pair("D", "low, sweet ").unwrap(),
                Node::chord_text_pair("G", "chari").unwrap(),
                Node::chord_text_pair("D", "ot.").unwrap(),
            ],
        ),
    ])
}

pub fn get_test_metadata() -> MetaInformation {
    MetaInformation {
        title: Some("Great new song".to_owned()),
        subtitle: Some("Originally known as 'Swing low sweet chariot'".to_owned()),
        artist: Some("Me".to_owned()),
        composer: Some("Wallace Willis".to_owned()),
        lyricist: Some("Wallace Willis".to_owned()),
        copyright: None,
        album: None,
        year: Some("1865".to_owned()),
        key: None,
        key_raw: None,
        original_key: None,
        original_key_raw: None,
        time: None,
        tempo: None,
        duration: None,
        capo: Some("1".to_owned()),
        original_title: None,
        alternative_title: None,
        ccli_song_id: None,
        b_notation: BNotation::B,
    }
}

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
            include_str!("../tests/resources/catalog/song-1.chorddown"),
        ),
        Song::new(
            SongMeta::new("song-2".into(), "Song 2".into(), FileType::Chorddown),
            include_str!("../tests/resources/catalog/song-2.chorddown"),
        ),
        Song::new(
            SongMeta::new("song-3".into(), "Song 3".into(), FileType::Chorddown),
            include_str!("../tests/resources/catalog/song-3.chorddown"),
        ),
        Song::new(
            SongMeta::new("song-40".into(), "Song 40".into(), FileType::Chorddown),
            include_str!("../tests/resources/swing_low_sweet_chariot.chorddown"),
        ),
        Song::new(
            SongMeta::new("song-50".into(), "Song 50".into(), FileType::Chorddown),
            include_str!("../tests/resources/german-test.chorddown"),
        ),
    ];
    Catalog::new("test-catalog", songs)
}
