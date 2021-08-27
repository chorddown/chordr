use std::fmt::{Display, Write};

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

    pub fn detect_modifier(&self) -> Option<Modifier> {
        match self {
            Lexeme::ChordStart | Lexeme::ChordEnd | Lexeme::QuoteStart | Lexeme::Colon => {
                Some(Modifier::None)
            }
            Lexeme::ChorusMark => Some(Modifier::Chorus),
            Lexeme::BridgeMark => Some(Modifier::Bridge),
            Lexeme::Literal(text) => {
                if text.is_empty() || text.trim().is_empty() {
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

    #[inline(always)]
    pub(crate) fn as_char(&self) -> char {
        match self {
            Lexeme::Newline => super::keywords::NEWLINE,
            Lexeme::ChordStart => super::keywords::CHORD_START,
            Lexeme::ChordEnd => super::keywords::CHORD_END,
            Lexeme::HeaderStart => super::keywords::HEADER_START,
            Lexeme::QuoteStart => super::keywords::QUOTE_START,
            Lexeme::Colon => super::keywords::COLON,
            Lexeme::ChorusMark => super::keywords::CHORUS_MARK,
            Lexeme::BridgeMark => super::keywords::BRIDGE_MARK,
            Lexeme::Literal(_) => unimplemented!("Lexeme::Literal does not support `as_char()`"),
            Lexeme::Eof => unimplemented!("Lexeme::Eof does not support `as_char()`"),
        }
    }
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Lexeme::Literal(inner) = self {
            write!(f, "{}", inner)
        } else if let Lexeme::Eof = self {
            Ok(())
        } else {
            f.write_char(match self {
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
            })
        }
    }
}
