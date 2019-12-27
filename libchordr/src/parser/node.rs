use crate::parser::section_type::SectionType;
use crate::tokenizer::{Modifier, Token};

#[derive(PartialOrd, PartialEq, Debug)]
pub enum Node {
    ChordTextPair {
        chord: Token,
        text: Token,
    },
    ChordStandalone(Token),
    Text(Token),
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

    pub fn chord_standalone<S: Into<String>>(value: S) -> Self {
        Node::ChordStandalone(Token::chord(value.into()))
    }
    pub fn chord_text_pair<S1: Into<String>, S2: Into<String>>(chord: S1, text: S2) -> Self {
        let chord = Token::chord(chord);
        let text = Token::literal(text);
        Node::ChordTextPair { chord, text }
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
