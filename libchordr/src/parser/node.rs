use std::convert::TryFrom;

use crate::error::Error;
use crate::models::chord::Chords;
use crate::models::meta::BNotation;
use crate::models::structure::{Section, SectionProvider};
use crate::modification::transposition::TransposableTrait;
use crate::parser::section_type::SectionType;
use crate::tokenizer::{Meta, Modifier, Token};

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum Node {
    ChordTextPair {
        chords: Chords,
        text: Token,
        last_in_line: bool,
    },
    ChordStandalone(Chords),
    Text(Token),
    Meta(Meta),

    Document(Vec<Node>),
    Headline(Token),
    Quote(Token),
    Section {
        head: Box<Node>,
        section_type: SectionType,
        children: Vec<Node>,
    },
    Newline,
}

impl Node {
    pub(crate) fn section<S: Into<String>, T: Into<SectionType> + Into<Modifier>>(
        level: u8,
        value: S,
        section_type: T,
        children: Vec<Node>,
    ) -> Self {
        let section_type: SectionType = section_type.into();

        Node::Section {
            head: Box::new(Node::headline(level, value, section_type.into())),
            section_type,
            children,
        }
    }

    pub(crate) fn headline<S: Into<String>>(level: u8, value: S, modifier: Modifier) -> Self {
        Node::Headline(Token::headline(level, value.into(), modifier))
    }

    #[allow(unused)]
    pub(crate) fn chord_standalone<S: AsRef<str>>(value: S) -> Result<Self, Error> {
        Ok(Node::ChordStandalone(Chords::try_from(
            value.as_ref(),
            BNotation::B,
        )?))
    }

    pub(crate) fn chord_text_pair<S1: AsRef<str>, S2: Into<String>>(
        chords: S1,
        text: S2,
    ) -> Result<Self, Error> {
        let chords = Chords::try_from(chords.as_ref(), BNotation::B)?;
        let text = Token::literal(text);

        Ok(Node::ChordTextPair {
            chords,
            text,
            last_in_line: false,
        })
    }

    pub(crate) fn chord_text_pair_last_in_line<S1: AsRef<str>, S2: Into<String>>(
        chords: S1,
        text: S2,
    ) -> Result<Self, Error> {
        let chords = Chords::try_from(chords.as_ref(), BNotation::B)?;
        let text = Token::literal(text);

        Ok(Node::ChordTextPair {
            chords,
            text,
            last_in_line: true,
        })
    }

    pub(crate) fn meta<S: AsRef<str>>(meta: S) -> Result<Self, Error> {
        match Meta::try_from(meta.as_ref()) {
            Ok(m) => Ok(Node::Meta(m)),
            Err(_) => Err(Error::parser_error(format!(
                "Invalid meta data given: '{}'",
                meta.as_ref()
            ))),
        }
    }

    pub(crate) fn newline() -> Self {
        Node::Newline
    }

    pub(crate) fn text<S: Into<String>>(value: S) -> Self {
        Node::Text(Token::literal(value.into()))
    }

    pub(crate) fn quote<S: Into<String>>(value: S) -> Self {
        Node::Quote(Token::quote(value))
    }

    pub fn get_sections(&self) -> Vec<Section> {
        SectionProvider::get_sections(self)
    }
}

impl TransposableTrait for Node {
    fn transpose(self, semitones: isize) -> Self {
        match self {
            Node::ChordTextPair {
                chords,
                text,
                last_in_line,
            } => Node::ChordTextPair {
                chords: chords.transpose(semitones),
                text,
                last_in_line,
            },
            Node::ChordStandalone(chords) => Node::ChordStandalone(chords.transpose(semitones)),

            Node::Document(nodes) => {
                Node::Document(nodes.into_iter().map(|n| n.transpose(semitones)).collect())
            }
            Node::Section {
                head,
                section_type,
                children,
            } => Node::Section {
                head,
                section_type,
                children: (children
                    .into_iter()
                    .map(|n| n.transpose(semitones))
                    .collect()),
            },
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::get_test_ast_small;

    use super::*;

    #[test]
    fn transpose_test() {
        let input = get_test_ast_small();
        let expected = Node::Document(vec![
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
                    Node::chord_text_pair("E", "low, sweet ").unwrap(),
                    Node::chord_text_pair("A", "chari").unwrap(),
                    Node::chord_text_pair("E", "ot,").unwrap(),
                    Node::text("Comin’ for to carry me "),
                    Node::chord_text_pair("B7", "home.").unwrap(),
                    Node::text("Swing "),
                    Node::chord_standalone("E7").unwrap(),
                ],
            ),
            Node::section(
                2,
                "Verse",
                Modifier::None,
                vec![
                    Node::chord_text_pair("F#", "low, sweet ").unwrap(),
                    Node::chord_text_pair("A", "chari").unwrap(),
                    Node::chord_text_pair("E", "ot,").unwrap(),
                    Node::chord_standalone("F#").unwrap(),
                    Node::chord_standalone("B").unwrap(),
                    Node::newline(),
                    Node::chord_standalone("C").unwrap(),
                    Node::chord_standalone("C#").unwrap(),
                ],
            ),
        ]);

        assert_eq!(input.transpose(2), expected);
    }
}
