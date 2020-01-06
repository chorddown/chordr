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
