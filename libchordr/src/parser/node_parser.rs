pub use super::meta_information::MetaInformation;
pub use super::*;
pub use super::node::Node;
pub use super::parser_result::ParserResult;
pub use super::section_type::SectionType;
use crate::tokenizer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::models::chord::Chord;

pub struct NodeParser {
    b_notation: BNotation
}

impl ParserTrait for NodeParser {
    type Result = Node;

    fn parse(&mut self, tokens: Vec<Token>) -> Self::Result {
        let mut tokens_iterator = tokens.into_iter().peekable();

        let mut elements = vec![];

        while let Some(token) = tokens_iterator.next() {
            elements.push(self.visit(token, &mut tokens_iterator));
        }

        Node::Document(elements)
    }
}

impl NodeParser {
    pub fn with_b_notation(b_notation: BNotation) -> Self {
        Self { b_notation }
    }

    fn visit(&mut self, token: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Node {
        match token {
            Token::Chord(_) => self.visit_chord(token, tokens),
            Token::Headline {
                level,
                text: _,
                modifier,
            } => {
                if level == 1 {}
                let head = Some(Box::new(Node::Headline(token)));

                if tokens.peek().is_some() {
                    // Collect children
                    let mut children = vec![];
                    while let Some(token) = tokens.peek() {
                        if token_is_start_of_section(token) {
                            break;
                        }
                        children.push(self.visit(tokens.next().unwrap(), tokens));
                    }

                    Node::Section {
                        head,
                        children,
                        section_type: modifier.into(),
                    }
                } else {
                    Node::Section {
                        head,
                        children: vec![],
                        section_type: modifier.into(),
                    }
                }
            }
            Token::Meta(meta) => {
                Node::Meta(meta)
            }
            Token::Literal(_) => Node::Text(token),
            Token::Quote(_) => Node::Quote(token),
            Token::Newline => Node::Newline,
        }
    }

    fn visit_chord(&mut self, token: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Node {
        let chords_raw = if let Token::Chord(c) = token { c } else { unreachable!("Invalid Token given") };

        let chords: Vec<Chord> = chords_raw
            .split('/')
            .filter_map(|r| Chord::try_from(&r, self.b_notation).ok())
            .collect();

        if let Some(next) = tokens.peek() {
            if let Token::Literal(_) = next {
                // Consume the next token
                let text = tokens.next().unwrap();

                return Node::ChordTextPair {
                    chords,
                    text,
                };
            }
        }

        return Node::ChordStandalone(chords);
    }
}

fn token_is_start_of_section(token: &Token) -> bool {
    match token {
        Token::Headline {
            level: _,
            text: _,
            modifier: _,
        } => true,
        Token::Quote(_) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Modifier;

    #[test]
    fn test_parse() {
        let mut parser = NodeParser::with_b_notation(BNotation::B);
        let ast = parser.parse(vec![
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
        ]);

        let expected_ast = Node::Document(vec![
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
                    Node::chord_text_pair("D", "low, sweet "),
                    Node::chord_text_pair("G", "chari"),
                    Node::chord_text_pair("D", "ot,"),
                    Node::text("Comin’ for to carry me "),
                    Node::chord_text_pair("A7", "home."),
                    Node::text("Swing "),
                    Node::chord_standalone("D7"),
                ],
            ),
            Node::section(
                2,
                "Verse",
                Modifier::None,
                vec![
                    Node::chord_text_pair("E", "low, sweet "),
                    Node::chord_text_pair("G", "chari"),
                    Node::chord_text_pair("D", "ot,"),
                    Node::chord_standalone("E"),
                    Node::chord_standalone("A"),
                    Node::newline(),
                    Node::chord_standalone("B"),
                    Node::chord_standalone("H"),
                ],
            ),
        ]);

        assert_eq!(expected_ast, ast);
    }
}
