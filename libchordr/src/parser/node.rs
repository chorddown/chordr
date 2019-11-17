use crate::tokenizer::Token;

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
        children: Vec<Node>,
    },
    Newline,
}

impl Node {
    pub fn section<S: Into<String>>(level: u8, value: S, children: Vec<Node>) -> Self {
        Node::Section {
            head: Some(Box::new(Node::headline(level, value))),
            children,
        }
    }

    pub fn headline<S: Into<String>>(level: u8, value: S) -> Self {
        Node::Headline(Token::headline(level, value.into()))
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
