use crate::models::meta::BNotation;
use crate::parser::{MetaInformation, Node};
use crate::tokenizer::{Modifier, Token, TokenLine};

#[cfg(test)]
pub fn get_test_tokens() -> Vec<TokenLine> {
    vec![
        vec![
            Token::headline(1, "Swing Low Sweet Chariot", Modifier::None),
            Token::newline(),
        ],
        vec![
            Token::headline(2, "Chorus", Modifier::Chorus),
            Token::newline(),
        ],
        vec![
            Token::literal("Swing "),
            Token::chord("D"),
            Token::literal("low, sweet "),
            Token::chord("G"),
            Token::literal("chari"),
            Token::chord("D"),
            Token::literal("ot,"),
            Token::newline(),
        ],
        vec![
            Token::literal("Comin’ for to carry me "),
            Token::chord("A7"),
            Token::literal("home."),
            Token::newline(),
        ],
        vec![
            Token::literal("Swing "),
            Token::chord("D7"),
            Token::literal("low, sweet "),
            Token::chord("G"),
            Token::literal("chari"),
            Token::chord("D"),
            Token::literal("ot,"),
            Token::newline(),
        ],
        vec![
            Token::literal("Comin’ for to "),
            Token::chord("A7"),
            Token::literal("carry me "),
            Token::chord("D"),
            Token::literal("home."),
            Token::newline(),
        ],
        vec![
            Token::headline(2, "Verse 1", Modifier::None),
            Token::newline(),
        ],
        vec![
            Token::literal("I "),
            Token::chord("D"),
            Token::literal("looked over Jordan, and "),
            Token::chord("G"),
            Token::literal("what did I "),
            Token::chord("D"),
            Token::literal("see,"),
            Token::newline(),
        ],
        vec![
            Token::literal("Comin’ for to carry me "),
            Token::chord("A7"),
            Token::literal("home."),
            Token::newline(),
        ],
        vec![
            Token::literal("A "),
            Token::chord("D"),
            Token::literal("band of angels "),
            Token::chord("G"),
            Token::literal("comin’ after "),
            Token::chord("D"),
            Token::literal("me,"),
            Token::newline(),
        ],
        vec![
            Token::literal("Comin’ for to "),
            Token::chord("A7"),
            Token::literal("carry me "),
            Token::chord("D"),
            Token::literal("home."),
            Token::newline(),
        ],
        vec![Token::quote("Chorus"), Token::newline()],
    ]
}

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
                Node::chord_text_pair("D", "ot,").unwrap(),
                Node::newline(),
                Node::text("Comin’ for to carry me "),
                Node::chord_text_pair("A7", "home.").unwrap(),
                Node::newline(),
                Node::text("Swing "),
                Node::chord_text_pair("D7", "low, sweet ").unwrap(),
                Node::chord_text_pair("G", "chari").unwrap(),
                Node::chord_text_pair("D", "ot,").unwrap(),
                Node::newline(),
                Node::text("Comin’ for to "),
                Node::chord_text_pair("A7", "carry me ").unwrap(),
                Node::chord_text_pair("D", "home.").unwrap(),
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
                Node::chord_text_pair("D", "see,").unwrap(),
                Node::newline(),
                Node::text("Comin’ for to carry me "),
                Node::chord_text_pair("A7", "home.").unwrap(),
                Node::newline(),
                Node::text("A "),
                Node::chord_text_pair("D", "band of angels ").unwrap(),
                Node::chord_text_pair("G", "comin’ after ").unwrap(),
                Node::chord_text_pair("D", "me,").unwrap(),
                Node::newline(),
                Node::text("Comin’ for to "),
                Node::chord_text_pair("A7", "carry me ").unwrap(),
                Node::chord_text_pair("D", "home.").unwrap(),
                Node::newline(),
            ],
        ),
        Node::quote("Chorus"),
        Node::newline(),
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
        time: None,
        tempo: None,
        duration: None,
        capo: Some("1".to_owned()),
        b_notation: BNotation::B,
    }
}
