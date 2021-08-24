use std::fmt::Display;

use crate::tokenizer::Modifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme {
    Newline,
    ChordStart,
    ChordEnd,
    HeaderStart,
    QuoteStart,
    Colon,
    ChorusMark,
    BridgeMark,
    Literal(String),
    Eof,
}

impl Lexeme {
    pub(super) fn literal<S: Into<String>>(s: S) -> Self {
        Lexeme::Literal(s.into())
    }

    pub fn to_string(&self) -> String {
        match self {
            Lexeme::Newline => super::keywords::NEWLINE.to_string(),
            Lexeme::ChordStart => super::keywords::CHORD_START.to_string(),
            Lexeme::ChordEnd => super::keywords::CHORD_END.to_string(),
            Lexeme::HeaderStart => super::keywords::HEADER_START.to_string(),
            Lexeme::QuoteStart => super::keywords::QUOTE_START.to_string(),
            Lexeme::Colon => super::keywords::COLON.to_string(),
            Lexeme::ChorusMark => super::keywords::CHORUS_MARK.to_string(),
            Lexeme::BridgeMark => super::keywords::BRIDGE_MARK.to_string(),
            Lexeme::Literal(inner) => inner.clone(),
            Lexeme::Eof => "".to_owned(),
        }
    }

    pub fn detect_modifier(&self) -> Option<Modifier> {
        match self {
            Lexeme::ChordStart | Lexeme::ChordEnd | Lexeme::QuoteStart | Lexeme::Colon => {
                Some(Modifier::None)
            }
            Lexeme::ChorusMark => Some(Modifier::Chorus),
            Lexeme::BridgeMark => Some(Modifier::Bridge),
            Lexeme::Literal(text) => {
                if text.is_empty() {
                    None
                } else if text.trim().is_empty() {
                    None
                } else {
                    Some(Modifier::None)
                }
            }
            Lexeme::HeaderStart => None,
            Lexeme::Newline => {
                // unreachable?
                None
            }
            Lexeme::Eof => {
                // unreachable?
                None
            }
        }
    }
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Lexeme::Literal(inner) = self {
            write!(f, "{}", inner)
        } else if let Lexeme::Eof = self {
            write!(f, "")
        } else {
            write!(
                f,
                "{}",
                match self {
                    Lexeme::Newline => super::keywords::NEWLINE,
                    Lexeme::ChordStart => super::keywords::CHORD_START,
                    Lexeme::ChordEnd => super::keywords::CHORD_END,
                    Lexeme::HeaderStart => super::keywords::HEADER_START,
                    Lexeme::QuoteStart => super::keywords::QUOTE_START,
                    Lexeme::Colon => super::keywords::COLON,
                    Lexeme::ChorusMark => super::keywords::CHORUS_MARK,
                    Lexeme::BridgeMark => super::keywords::BRIDGE_MARK,
                    Lexeme::Literal(_) => unreachable!(),
                    Lexeme::Eof => unreachable!(),
                }
            )
        }
    }
}
