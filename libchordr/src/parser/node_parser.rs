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
    type OkType = Node;

    fn parse(&mut self, tokens: Vec<Token>) -> Result<Self::OkType, Error> {
        let mut tokens_iterator = tokens.into_iter().peekable();

        let mut elements = vec![];

        while let Some(token) = tokens_iterator.next() {
            elements.push(self.visit(token, &mut tokens_iterator)?);
        }

        Ok(Node::Document(elements))
    }
}

impl NodeParser {
    pub fn with_b_notation(b_notation: BNotation) -> Self {
        Self { b_notation }
    }

    fn visit(&mut self, token: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
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
                        let result = self.visit(tokens.next().unwrap(), tokens)?;
                        children.push(result);
                    }

                    Ok(Node::Section {
                        head,
                        children,
                        section_type: modifier.into(),
                    })
                } else {
                    Ok(Node::Section {
                        head,
                        children: vec![],
                        section_type: modifier.into(),
                    })
                }
            }
            Token::Meta(meta) => Ok(Node::Meta(meta)),
            Token::Literal(_) => Ok(Node::Text(token)),
            Token::Quote(_) => Ok(Node::Quote(token)),
            Token::Newline => Ok(Node::Newline),
        }
    }

    fn visit_chord(&mut self, token: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
        let chords_raw = if let Token::Chord(c) = token { c } else { unreachable!("Invalid Token given") };

        let chords: Vec<Chord> = chords_raw
            .split('/')
            .filter_map(|r| Chord::try_from(&r, self.b_notation).ok())
            .collect();

        if let Some(next) = tokens.peek() {
            if let Token::Literal(_) = next {
                // Consume the next token
                let text = tokens.next().unwrap();

                return Ok(Node::ChordTextPair {
                    chords,
                    text,
                });
            }
        }

        Ok(Node::ChordStandalone(chords))
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
        let result = parser.parse(get_test_parser_input());

        assert!(result.is_ok());
        let ast = result.unwrap();

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
