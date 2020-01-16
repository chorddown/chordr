use crate::parser::section_type::SectionType;
use crate::tokenizer::{Modifier, Token, Meta};
use crate::models::meta::BNotation;
use crate::error::Error;
use crate::models::chord::Chords;

#[derive(PartialOrd, PartialEq, Debug)]
pub enum Node {
    ChordTextPair {
        chords: Chords,
        text: Token,
    },
    ChordStandalone(Chords),
    Text(Token),
    Meta(Meta),

    Document(Vec<Node>),
    Headline(Token),
    Quote(Token),
    Section {
        head: Option<Box<Node>>,
        section_type: SectionType,
        children: Vec<Node>,
    },
    Newline,
}

impl Node {
    pub fn section<S: Into<String>, T: Into<SectionType> + Into<Modifier>>(
        level: u8,
        value: S,
        section_type: T,
        children: Vec<Node>,
    ) -> Self {
        let section_type: SectionType = section_type.into();

        Node::Section {
            head: Some(Box::new(Node::headline(level, value, section_type.into()))),
            section_type,
            children,
        }
    }

    pub fn headline<S: Into<String>>(level: u8, value: S, modifier: Modifier) -> Self {
        Node::Headline(Token::headline(level, value.into(), modifier))
    }

    pub fn chord_standalone<S: AsRef<str>>(value: S) -> Result<Self, Error> {
        Ok(Node::ChordStandalone(Chords::try_from(value.as_ref(), BNotation::B)?))
    }

    pub fn chord_text_pair<S1: AsRef<str>, S2: Into<String>>(chords: S1, text: S2) -> Result<Self, Error> {
        let chords = Chords::try_from(chords.as_ref(), BNotation::B)?;
        let text = Token::literal(text);

        Ok(Node::ChordTextPair { chords, text })
    }

    pub fn newline() -> Self {
        Node::Newline
    }

    pub fn text<S: Into<String>>(value: S) -> Self {
        Node::Text(Token::literal(value.into()))
    }

    pub fn quote<S: Into<String>>(value: S) -> Self {
        Node::Quote(Token::quote(value))
    }
}
