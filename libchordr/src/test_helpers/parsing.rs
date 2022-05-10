use crate::models::meta::BNotation;
use crate::parser::{MetaInformation, Node, SectionType};
use crate::tokenizer::{Modifier, Token};

/// Return a small example set of Tokens
///
/// See also [get_test_ast_small()]
#[cfg(test)]
pub fn get_test_tokens_small() -> Vec<Token> {
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

/// Return a short example AST
/// This is also the excepted parsed AST for `get_test_parser_input()`
///
/// See also [get_test_tokens_small()]
#[cfg(test)]
pub fn get_test_ast_small() -> Node {
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
                Node::text("Swing "),
                Node::chord_text_pair("D", "low, sweet ").unwrap(),
                Node::chord_text_pair("G", "chari").unwrap(),
                Node::chord_text_pair("D", "ot,").unwrap(),
                Node::text("Comin’ for to carry me "),
                Node::chord_text_pair("A7", "home.").unwrap(),
                Node::text("Swing "),
                Node::chord_standalone("D7").unwrap(),
            ],
        ),
        Node::section(
            2,
            "Verse",
            Modifier::None,
            vec![
                Node::chord_text_pair("E", "low, sweet ").unwrap(),
                Node::chord_text_pair("G", "chari").unwrap(),
                Node::chord_text_pair("D", "ot,").unwrap(),
                Node::chord_standalone("E").unwrap(),
                Node::chord_standalone("A").unwrap(),
                Node::newline(),
                Node::chord_standalone("A#").unwrap(),
                Node::chord_standalone("H").unwrap(),
            ],
        ),
    ])
}

/// Return the complete set of Tokens for swing_low_sweet_chariot.chorddown
///
/// See [../tests/resources/swing_low_sweet_chariot.chorddown]
/// See also [get_test_ast()]
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

/// Return the complete AST for swing_low_sweet_chariot.chorddown
///
/// See [../tests/resources/swing_low_sweet_chariot.chorddown]
/// See also [get_test_tokens()]
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
        Node::Section {
            head: Box::new(Node::quote("Chorus")),
            section_type: SectionType::Reference,
            children: vec![],
        },
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
        Node::Section {
            head: Box::new(Node::quote("Chorus 2x")),
            section_type: SectionType::Reference,
            children: vec![],
        },
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
