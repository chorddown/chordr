mod meta_information;
mod node;
mod parser_result;
mod section_type;

pub use self::meta_information::MetaInformation;
pub use self::node::Node;
pub use self::parser_result::ParserResult;
pub use self::section_type::SectionType;
use crate::models::meta::*;
use crate::tokenizer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    meta: MetaInformation,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            meta: MetaInformation::default(),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> ParserResult {
        let mut tokens_iterator = tokens.into_iter().peekable();

        let mut elements = vec![];

        while let Some(token) = tokens_iterator.next() {
            elements.push(self.visit(token, &mut tokens_iterator));
        }

        ParserResult::new(Node::Document(elements), self.meta.clone())
    }

    fn visit(&mut self, token: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Node {
        match token {
            Token::Chord(_) => self.visit_chord(token, tokens),
            Token::Headline {
                level,
                ref text,
                modifier,
            } => {
                if level == 1 {
                    self.meta.title = Some(text.clone())
                }
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
                self.meta.assign_from_token(&meta);
                Node::Meta(meta)
            }
            Token::Literal(_) => Node::Text(token),
            Token::Quote(_) => Node::Quote(token),
            Token::Newline => Node::Newline,
        }
    }

    fn visit_chord(&mut self, token: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Node {
        let chords = if let Token::Chord(c) = token { c } else { unreachable!("Invalid Token given") };

        if BNotation::is_european_chord(&chords) {
            self.meta.b_notation = BNotation::H;
        }

        if let Some(next) = tokens.peek() {
            if let Token::Literal(_) = next {
                // Consume the next token
                let text = tokens.next().unwrap();

                return Node::ChordTextPair {
                    chord: Token::Chord(chords),
                    text,
                };
            }
        }

        return Node::ChordStandalone(Token::Chord(chords));
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
        let mut parser = Parser::new();
        let result = parser.parse(vec![
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

        assert_eq!(
            Some("Swing Low Sweet Chariot".to_string()),
            result.meta().title
        );

        let ast = result.node();

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

    #[test]
    fn test_detect_b_notation() {
        let mut parser = Parser::new();
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with standard B notation w/ text", Modifier::None),
                Token::newline(),
                Token::chord("E"),
                Token::literal("A text"),
            ]);

            assert_eq!(result.meta_as_ref().b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with standard B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("E"),
            ]);

            assert_eq!(result.meta_as_ref().b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/ text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
                Token::literal("A text"),
            ]);

            assert_eq!(result.meta_as_ref().b_notation, BNotation::H);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
            ]);

            assert_eq!(result.meta_as_ref().b_notation, BNotation::H);
        }
    }
}
