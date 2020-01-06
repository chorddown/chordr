use crate::tokenizer::modifier::Modifier;
use super::meta::Meta;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Token {
    Headline {
        level: u8,
        text: String,
        modifier: Modifier,
    },
    Literal(String),
    Meta(Meta),
    Chord(String),
    Quote(String),
    Newline,
}

impl Token {
    pub fn headline<S: Into<String>>(level: u8, value: S, modifier: Modifier) -> Self {
        let text = value.into();

        Token::Headline {
            level,
            text,
            modifier,
        }
    }

    pub fn chord<S: Into<String>>(value: S) -> Self {
        Token::Chord(value.into())
    }

    pub fn newline() -> Self {
        Token::Newline
    }

    pub fn literal<S: Into<String>>(value: S) -> Self {
        Token::Literal(value.into())
    }

    pub fn quote<S: Into<String>>(value: S) -> Self {
        Token::Quote(value.into())
    }
}
